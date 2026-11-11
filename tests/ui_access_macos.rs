//! The macOS accessibility adapter, asked what a screen reader asks.
//!
//! An `NSView` with no window gets the adapter; the test then calls the
//! `NSAccessibility` methods VoiceOver calls (children, role, title, value,
//! enabled, focused, press, increment, set value) on the view and on the
//! elements it hands out, and walks the sample menu with them. No window is
//! opened and no screen reader runs: what this does not cover is VoiceOver's
//! own speech and cursor, which need a visible window.
//!
//! AppKit must be used from the main thread, so this is a program without
//! the test harness: each check below is a function named after what must
//! hold, run in order by `main`. Off macOS it does nothing.

#[cfg(not(target_os = "macos"))]
fn main() {}

#[cfg(target_os = "macos")]
fn main() {
    mac::run();
}

/// The guide's sample menu, `examples/ui_menu.rs`: its nodes and what it
/// does with an event, without its `main`.
#[cfg(target_os = "macos")]
#[allow(dead_code)]
#[path = "../examples/ui_menu.rs"]
mod sample;

#[cfg(target_os = "macos")]
mod mac {
    use objc2::rc::Id;
    use objc2::runtime::{AnyObject, NSObject};
    use objc2::{class, msg_send, msg_send_id};
    use objc2_foundation::{NSArray, NSNumber, NSString};
    use ponko::prelude::*;
    use ponko::ui::{Ui, UiAccessAdapter, UiEvent, UiId, UiInput, UiLayout};

    const VIEW: Vec2 = Vec2::new(800.0, 600.0);

    // ------------------------------------------- what a screen reader asks

    /// An accessibility element: the view, or a node it handed out.
    type Element = Id<AnyObject>;

    fn children(of: &AnyObject) -> Vec<Element> {
        // SAFETY: `accessibilityChildren` returns an array of objects (or nil).
        let array: Option<Id<NSArray<NSObject>>> = unsafe { msg_send_id![of, accessibilityChildren] };
        let Some(array) = array else { return Vec::new() };
        // SAFETY: plain `NSArray` messages, with indices below its count.
        let count: usize = unsafe { msg_send![&*array, count] };
        (0..count)
            .map(|i| unsafe { msg_send_id![&*array, objectAtIndex: i] })
            .collect()
    }

    fn role(of: &AnyObject) -> String {
        // SAFETY: returns an `NSString` or nil.
        let role: Option<Id<NSString>> = unsafe { msg_send_id![of, accessibilityRole] };
        role.map(|r| r.to_string()).unwrap_or_default()
    }

    fn title(of: &AnyObject) -> Option<String> {
        // SAFETY: returns an `NSString` or nil.
        let title: Option<Id<NSString>> = unsafe { msg_send_id![of, accessibilityTitle] };
        title.map(|t| t.to_string())
    }

    fn help(of: &AnyObject) -> Option<String> {
        // SAFETY: returns an `NSString` or nil.
        let help: Option<Id<NSString>> = unsafe { msg_send_id![of, accessibilityHelp] };
        help.map(|t| t.to_string())
    }

    /// What `accessibilityValue` answers: text, a number, or nothing.
    #[derive(Debug, PartialEq)]
    enum Value {
        None,
        Text(String),
        Number(f64),
    }

    fn value(of: &AnyObject) -> Value {
        // SAFETY: returns an object (a string or a number here) or nil.
        let value: Option<Id<NSObject>> = unsafe { msg_send_id![of, accessibilityValue] };
        let Some(value) = value else { return Value::None };
        // SAFETY: `isKindOfClass:` is answered by every object.
        let is_text: bool = unsafe { msg_send![&*value, isKindOfClass: class!(NSString)] };
        if is_text {
            // SAFETY: just checked that the object is an `NSString`.
            let text: &NSString = unsafe { &*(Id::as_ptr(&value).cast::<NSString>()) };
            return Value::Text(text.to_string());
        }
        // SAFETY: as above.
        let is_number: bool = unsafe { msg_send![&*value, isKindOfClass: class!(NSNumber)] };
        assert!(is_number, "a value is text or a number");
        // SAFETY: just checked that the object is an `NSNumber`.
        let number: &NSNumber = unsafe { &*(Id::as_ptr(&value).cast::<NSNumber>()) };
        Value::Number(number.as_f64())
    }

    fn enabled(of: &AnyObject) -> bool {
        // SAFETY: returns a BOOL.
        unsafe { msg_send![of, isAccessibilityEnabled] }
    }

    fn focused(of: &AnyObject) -> bool {
        // SAFETY: returns a BOOL.
        unsafe { msg_send![of, isAccessibilityFocused] }
    }

    fn modal(of: &AnyObject) -> bool {
        // SAFETY: returns a BOOL.
        unsafe { msg_send![of, isAccessibilityModal] }
    }

    /// VoiceOver's "press" (VO-Space). True if the element took it.
    fn press(of: &AnyObject) -> bool {
        // SAFETY: returns a BOOL.
        unsafe { msg_send![of, accessibilityPerformPress] }
    }

    fn increment(of: &AnyObject) -> bool {
        // SAFETY: returns a BOOL.
        unsafe { msg_send![of, accessibilityPerformIncrement] }
    }

    fn decrement(of: &AnyObject) -> bool {
        // SAFETY: returns a BOOL.
        unsafe { msg_send![of, accessibilityPerformDecrement] }
    }

    fn set_focused(of: &AnyObject) {
        // SAFETY: takes a BOOL, returns nothing.
        unsafe { msg_send![of, setAccessibilityFocused: true] }
    }

    fn set_text(of: &AnyObject, text: &str) {
        let text = NSString::from_str(text);
        // SAFETY: takes an object, returns nothing.
        unsafe { msg_send![of, setAccessibilityValue: &*text] }
    }

    fn set_number(of: &AnyObject, number: f64) {
        let number = NSNumber::new_f64(number);
        // SAFETY: takes an object, returns nothing.
        unsafe { msg_send![of, setAccessibilityValue: &*number] }
    }

    /// The element the system would say has keyboard focus.
    fn focused_element(view: &AnyObject) -> Option<Element> {
        // SAFETY: returns an object or nil.
        unsafe { msg_send_id![view, accessibilityFocusedUIElement] }
    }

    /// Every element under `of`, depth-first.
    fn all(of: &AnyObject, out: &mut Vec<Element>) {
        for child in children(of) {
            all(&child, out);
            out.push(child);
        }
    }

    /// The one element with this role and title (or, for static text, value).
    fn find(view: &AnyObject, want_role: &str, name: &str) -> Element {
        try_find(view, want_role, name)
            .unwrap_or_else(|| panic!("no {want_role} named {name:?} in:\n{}", outline(view)))
    }

    fn try_find(view: &AnyObject, want_role: &str, name: &str) -> Option<Element> {
        let mut elements = Vec::new();
        all(view, &mut elements);
        let mut found = elements.into_iter().filter(|e| {
            role(e) == want_role && (title(e).as_deref() == Some(name) || value(e) == Value::Text(name.to_string()))
        });
        let first = found.next();
        assert!(found.next().is_none(), "more than one {want_role} named {name:?}");
        first
    }

    /// The tree as text, one element per line: what the checks compare.
    fn outline(view: &AnyObject) -> String {
        fn line(of: &AnyObject, depth: usize, out: &mut String) {
            use std::fmt::Write;
            write!(out, "{:indent$}{}", "", role(of), indent = depth * 2).unwrap();
            if let Some(title) = title(of) {
                write!(out, " {title:?}").unwrap();
            }
            match value(of) {
                Value::None => {}
                Value::Text(text) => write!(out, " value={text:?}").unwrap(),
                Value::Number(n) => write!(out, " value={n:.2}").unwrap(),
            }
            if let Some(help) = help(of) {
                write!(out, " help={help:?}").unwrap();
            }
            if !enabled(of) {
                out.push_str(" disabled");
            }
            if focused(of) {
                out.push_str(" focused");
            }
            out.push('\n');
            for child in children(of) {
                line(&child, depth + 1, out);
            }
        }
        let mut out = String::new();
        for child in children(view) {
            line(&child, 0, &mut out);
        }
        out
    }

    // ------------------------------------------------------ the sample menu

    struct Menu {
        /// Kept alive for the adapter.
        view: Element,
        adapter: UiAccessAdapter,
        ui: Ui,
        root: UiId,
        play: UiId,
        volume: UiId,
        name: UiId,
        music: UiId,
        quit: UiId,
    }

    impl Menu {
        /// A view with the adapter and the sample menu, before any
        /// assistive technology has asked for it.
        fn asleep() -> Self {
            // SAFETY: `+[NSView new]` on the main thread returns a view.
            let view: Element = unsafe { msg_send_id![class!(NSView), new] };
            // SAFETY: the view is alive (the menu keeps it), this is the main
            // thread, and the view has no other adapter.
            let adapter = unsafe { UiAccessAdapter::for_ns_view(Id::as_ptr(&view).cast_mut().cast(), "Sample menu") };
            let mut ui = Ui::default();
            let root = ui.root().layout(UiLayout::fill().padding(10.0).gap(4.0)).id();
            ui.label(root, "Settings").layout(UiLayout::row(24.0));
            let play = ui.button(root, "Play").layout(UiLayout::row(30.0)).id();
            let volume = ui
                .slider(root)
                .layout(UiLayout::row(20.0))
                .value(0.5)
                .accessible_name("Volume")
                .id();
            let name = ui
                .text_input(root)
                .layout(UiLayout::row(30.0))
                .text("Ann")
                .accessible_name("Player name")
                .id();
            let music = ui.toggle(root, "Music").layout(UiLayout::row(30.0)).value(1.0).id();
            let quit = ui.button(root, "Quit").layout(UiLayout::row(30.0)).enabled(false).id();
            ui.push_screen(root, false);
            let mut menu = Self {
                view,
                adapter,
                ui,
                root,
                play,
                volume,
                name,
                music,
                quit,
            };
            menu.frame(UiInput::default());
            menu
        }

        /// The same, with a screen reader attached and the window focused.
        fn read() -> Self {
            let mut menu = Self::asleep();
            menu.adapter.set_window_focused(true);
            children(&menu.view);
            menu.frame(UiInput::default());
            menu
        }

        /// One frame as the engine runs it: the UI takes its input (and the
        /// requests queued last frame), then the adapter publishes.
        fn frame(&mut self, input: UiInput) -> Vec<UiEvent> {
            self.ui.frame(VIEW, &input);
            let events = self.ui.events().collect();
            self.adapter.update(&mut self.ui, 1.0);
            events
        }

        /// A request made through the system reaches the UI on the next
        /// frame; this runs it and returns that frame's events.
        fn after_request(&mut self) -> Vec<UiEvent> {
            self.adapter.update(&mut self.ui, 1.0);
            self.frame(UiInput::default())
        }
    }

    const MENU: &str = "\
AXGroup
  AXGroup
    AXStaticText value=\"Settings\"
    AXButton \"Play\"
    AXSlider \"Volume\" value=0.50
    AXTextField \"Player name\" value=\"Ann\"
    AXCheckBox \"Music\" value=1.00
    AXButton \"Quit\" disabled
";

    // ---------------------------------------------------------- the checks

    fn nothing_is_built_until_assistive_technology_asks() {
        let mut menu = Menu::asleep();
        for _ in 0..3 {
            menu.frame(UiInput::default());
        }
        assert!(!menu.adapter.is_active(), "no one asked yet");

        // The first question gets a placeholder; the next frame the tree.
        let first = children(&menu.view);
        assert!(menu.adapter.is_active(), "asking for the children wakes the adapter");
        assert_eq!(first.len(), 1);
        assert_eq!(children(&first[0]).len(), 0, "a placeholder until the next frame");
        menu.frame(UiInput::default());
        assert_eq!(outline(&menu.view), MENU);
    }

    fn the_sample_menu_reads_as_roles_names_values_and_states() {
        let menu = Menu::read();
        assert_eq!(outline(&menu.view), MENU);
        // The states a screen reader announces.
        assert!(enabled(&find(&menu.view, "AXButton", "Play")));
        assert!(!enabled(&find(&menu.view, "AXButton", "Quit")));
        assert_eq!(value(&find(&menu.view, "AXCheckBox", "Music")), Value::Number(1.0));
        assert_eq!(value(&find(&menu.view, "AXSlider", "Volume")), Value::Number(0.5));
        assert_eq!(
            value(&find(&menu.view, "AXTextField", "Player name")),
            Value::Text("Ann".into())
        );
        assert!(focused_element(&menu.view).is_none(), "nothing is focused yet");
    }

    fn pressing_a_button_through_the_system_clicks_it() {
        let mut menu = Menu::read();
        assert!(press(&find(&menu.view, "AXButton", "Play")));
        assert_eq!(menu.after_request(), [UiEvent::Clicked(menu.play)]);
        assert_eq!(
            menu.ui.focus(),
            Some(menu.play),
            "and it has focus, as after a key press"
        );
        assert!(focused(&find(&menu.view, "AXButton", "Play")));

        // A disabled button refuses, and nothing happens.
        assert!(!press(&find(&menu.view, "AXButton", "Quit")));
        assert_eq!(menu.after_request(), []);
        assert_eq!(menu.ui.focus(), Some(menu.play));
        let _ = menu.quit;
    }

    fn focus_moves_both_ways_between_the_keyboard_and_the_system() {
        let mut menu = Menu::read();
        // Tab, as the keyboard does it: the system sees the focused element.
        menu.frame(UiInput {
            next: true,
            ..Default::default()
        });
        let at = focused_element(&menu.view).expect("the system reports a focused element");
        assert_eq!((role(&at).as_str(), title(&at).as_deref()), ("AXButton", Some("Play")));
        assert!(outline(&menu.view).contains("AXButton \"Play\" focused\n"));

        // The screen reader moves focus: the UI follows, without an event.
        set_focused(&find(&menu.view, "AXTextField", "Player name"));
        assert_eq!(menu.after_request(), []);
        assert_eq!(menu.ui.focus(), Some(menu.name));
        let at = focused_element(&menu.view).unwrap();
        assert_eq!(title(&at).as_deref(), Some("Player name"));
        assert!(!focused(&find(&menu.view, "AXButton", "Play")));

        // And the keyboard carries on from there: Tab goes to the toggle.
        menu.frame(UiInput {
            next: true,
            ..Default::default()
        });
        assert_eq!(menu.ui.focus(), Some(menu.music));
        assert!(focused(&find(&menu.view, "AXCheckBox", "Music")));

        // A window that lost focus has no focused element.
        menu.adapter.set_window_focused(false);
        assert!(!focused(&find(&menu.view, "AXCheckBox", "Music")));
    }

    fn a_slider_steps_and_takes_values_through_the_system() {
        let mut menu = Menu::read();
        let slider = find(&menu.view, "AXSlider", "Volume");
        assert!(increment(&slider));
        assert_eq!(menu.after_request(), [UiEvent::Changed(menu.volume)]);
        assert!((menu.ui.value(menu.volume).unwrap() - 0.55).abs() < 1e-6);
        match value(&find(&menu.view, "AXSlider", "Volume")) {
            Value::Number(n) => assert!((n - 0.55).abs() < 1e-6, "{n}"),
            other => panic!("{other:?}"),
        }

        assert!(decrement(&slider));
        assert!(decrement(&slider));
        assert_eq!(
            menu.after_request(),
            [UiEvent::Changed(menu.volume), UiEvent::Changed(menu.volume)]
        );
        assert!((menu.ui.value(menu.volume).unwrap() - 0.45).abs() < 1e-6);

        set_number(&slider, 0.9);
        assert_eq!(menu.after_request(), [UiEvent::Changed(menu.volume)]);
        assert!((menu.ui.value(menu.volume).unwrap() - 0.9).abs() < 1e-6);

        // With focus on it the arrow keys still work, and the system hears
        // about it.
        set_focused(&slider);
        assert_eq!(menu.after_request(), []);
        menu.frame(UiInput {
            direction: vec2(1.0, 0.0),
            ..Default::default()
        });
        match value(&find(&menu.view, "AXSlider", "Volume")) {
            Value::Number(n) => assert!((n - 0.95).abs() < 1e-6, "{n}"),
            other => panic!("{other:?}"),
        }
    }

    fn a_toggle_flips_and_a_text_field_takes_text_through_the_system() {
        let mut menu = Menu::read();
        assert!(press(&find(&menu.view, "AXCheckBox", "Music")));
        assert_eq!(menu.after_request(), [UiEvent::Changed(menu.music)]);
        assert_eq!(menu.ui.value(menu.music), Some(0.0));
        assert_eq!(value(&find(&menu.view, "AXCheckBox", "Music")), Value::Number(0.0));

        set_text(&find(&menu.view, "AXTextField", "Player name"), "Bea");
        assert_eq!(menu.after_request(), [UiEvent::Changed(menu.name)]);
        assert_eq!(menu.ui.text(menu.name), Some("Bea"));
        assert_eq!(
            value(&find(&menu.view, "AXTextField", "Player name")),
            Value::Text("Bea".into())
        );

        // Typing still edits it, and the system hears the new text.
        set_focused(&find(&menu.view, "AXTextField", "Player name"));
        menu.after_request();
        menu.frame(UiInput {
            text: "!".into(),
            ..Default::default()
        });
        assert_eq!(
            value(&find(&menu.view, "AXTextField", "Player name")),
            Value::Text("Bea!".into())
        );
    }

    fn renaming_disabling_hiding_and_describing_update_what_is_read() {
        let mut menu = Menu::read();
        menu.ui.set_text(menu.play, "Continue");
        menu.ui.set_accessible_description(menu.play, "Picks up the last save");
        menu.ui.set_enabled(menu.quit, true);
        menu.frame(UiInput::default());
        let play = find(&menu.view, "AXButton", "Continue");
        assert_eq!(help(&play).as_deref(), Some("Picks up the last save"));
        assert!(try_find(&menu.view, "AXButton", "Play").is_none());
        assert!(enabled(&find(&menu.view, "AXButton", "Quit")));

        // Disabled: read, but it refuses a press.
        menu.ui.set_enabled(menu.play, false);
        menu.frame(UiInput::default());
        let play = find(&menu.view, "AXButton", "Continue");
        assert!(!enabled(&play) && !press(&play));
        assert_eq!(menu.after_request(), []);

        // Hidden: gone, with nothing left behind.
        let mut hidden: ponko::ui::layout::Style = UiLayout::row(20.0).into();
        hidden.display = ponko::ui::layout::Display::None;
        menu.ui.set_layout(menu.volume, hidden);
        menu.frame(UiInput::default());
        assert!(try_find(&menu.view, "AXSlider", "Volume").is_none());
        assert_eq!(children(&children(&children(&menu.view)[0])[0]).len(), 5);

        // Shown again.
        menu.ui.set_layout(menu.volume, UiLayout::row(20.0).into());
        menu.frame(UiInput::default());
        assert_eq!(value(&find(&menu.view, "AXSlider", "Volume")), Value::Number(0.5));
    }

    fn a_modal_dialog_replaces_the_menu_and_closing_it_brings_the_menu_back() {
        let mut menu = Menu::read();
        let dialog = menu.ui.root().layout(UiLayout::centered(300.0, 120.0)).id();
        menu.ui.label(dialog, "Quit the game?").layout(UiLayout::row(24.0));
        let yes = menu.ui.button(dialog, "Yes").layout(UiLayout::row(30.0)).id();
        menu.ui.push_screen(dialog, true);
        menu.frame(UiInput::default());

        assert!(
            try_find(&menu.view, "AXButton", "Play").is_none(),
            "the menu is covered"
        );
        let screens = children(&children(&menu.view)[0]);
        assert_eq!(screens.len(), 1);
        assert!(modal(&screens[0]), "the dialog says it is modal");
        assert!(press(&find(&menu.view, "AXButton", "Yes")));
        assert_eq!(menu.after_request(), [UiEvent::Clicked(yes)]);

        // Escape closes it, as it always did; the menu is read again.
        let events = menu.frame(UiInput {
            cancel: true,
            ..Default::default()
        });
        assert_eq!(events, [UiEvent::ScreenPopped(dialog)]);
        menu.frame(UiInput::default());
        assert!(try_find(&menu.view, "AXButton", "Yes").is_none());
        assert_eq!(outline(&menu.view), MENU);
        let _ = menu.root;
    }

    fn an_unchanged_node_keeps_its_element_when_another_one_changes() {
        let mut menu = Menu::read();
        let play_before = find(&menu.view, "AXButton", "Play");
        let slider_before = find(&menu.view, "AXSlider", "Volume");
        menu.ui.set_value(menu.volume, 0.25);
        menu.frame(UiInput::default());
        // The system keeps the elements it was given: only the slider's
        // value was sent, nothing was rebuilt.
        assert_eq!(
            Id::as_ptr(&find(&menu.view, "AXButton", "Play")),
            Id::as_ptr(&play_before)
        );
        assert_eq!(
            Id::as_ptr(&find(&menu.view, "AXSlider", "Volume")),
            Id::as_ptr(&slider_before)
        );
        assert_eq!(
            value(&slider_before),
            Value::Number(0.25),
            "the old handle reads the new value"
        );
    }

    fn a_request_for_a_node_that_is_gone_is_dropped() {
        let mut menu = Menu::read();
        let play = find(&menu.view, "AXButton", "Play");
        assert!(press(&play));
        // The button goes away before the request is applied.
        menu.ui.remove(menu.play);
        assert_eq!(menu.after_request(), []);
        assert!(try_find(&menu.view, "AXButton", "Play").is_none());
        assert!(!press(&play), "a stale element refuses");
    }

    /// `examples/ui_menu.rs`, the menu of the manual screen-reader pass,
    /// operated the way that pass operates it.
    fn the_example_menu_is_played_through_the_system_from_play_to_the_quit_dialog() {
        // SAFETY: `+[NSView new]` on the main thread returns a view.
        let view: Element = unsafe { msg_send_id![class!(NSView), new] };
        // SAFETY: the view outlives the adapter (declared first, dropped
        // last), this is the main thread, and the view has no other adapter.
        let mut adapter = unsafe { UiAccessAdapter::for_ns_view(Id::as_ptr(&view).cast_mut().cast(), "Settings") };
        let mut ui = Ui::default();
        let menu = crate::sample::build(&mut ui);
        // One frame of the example: the UI, the game's reaction to its
        // events, then the adapter.
        let frame = |ui: &mut Ui, adapter: &mut UiAccessAdapter| {
            ui.frame(VIEW, &UiInput::default());
            let mut quit = false;
            while let Some(event) = ui.next_event() {
                quit |= crate::sample::react(ui, &menu, event);
            }
            adapter.update(ui, 1.0);
            quit
        };
        frame(&mut ui, &mut adapter);
        adapter.set_window_focused(true);
        children(&view);
        frame(&mut ui, &mut adapter);
        assert_eq!(
            outline(&view),
            "\
AXGroup
  AXGroup
    AXStaticText value=\"Settings\"
    AXStaticText value=\"Ready\"
    AXButton \"Play\" help=\"Starts a new run\"
    AXSlider \"Volume\" value=0.50
    AXTextField \"Player name\" value=\"Ann\"
    AXCheckBox \"Music\" value=1.00
    AXButton \"Quit\"
"
        );

        // Play: the status line, a live region, now says who plays.
        assert!(press(&find(&view, "AXButton", "Play")));
        frame(&mut ui, &mut adapter);
        frame(&mut ui, &mut adapter);
        assert!(outline(&view).contains("AXStaticText value=\"Started a run for Ann\""));

        // A step of the slider is reported in the status line too.
        assert!(increment(&find(&view, "AXSlider", "Volume")));
        frame(&mut ui, &mut adapter);
        frame(&mut ui, &mut adapter);
        assert!(outline(&view).contains("AXStaticText value=\"Volume 55%\""));

        // Quit opens the dialog, which is all that is read; No closes it.
        // (The request reaches the UI on the next frame, the game opens the
        // dialog in that frame, and the frame after lays it out.)
        assert!(press(&find(&view, "AXButton", "Quit")));
        for _ in 0..3 {
            frame(&mut ui, &mut adapter);
        }
        assert!(try_find(&view, "AXButton", "Play").is_none());
        let screens = children(&children(&view)[0]);
        assert_eq!(screens.len(), 1);
        assert!(modal(&screens[0]));
        assert_eq!(title(&screens[0]).as_deref(), Some("Quit the game?"));
        assert!(press(&find(&view, "AXButton", "No")));
        for _ in 0..3 {
            frame(&mut ui, &mut adapter);
        }
        assert!(try_find(&view, "AXButton", "Play").is_some());

        // Yes is the one answer that quits.
        assert!(press(&find(&view, "AXButton", "Quit")));
        for _ in 0..3 {
            frame(&mut ui, &mut adapter);
        }
        assert!(press(&find(&view, "AXButton", "Yes")));
        frame(&mut ui, &mut adapter);
        assert!(frame(&mut ui, &mut adapter), "Yes quits the game");
    }

    pub fn run() {
        let checks: &[(&str, fn())] = &[
            (
                "nothing_is_built_until_assistive_technology_asks",
                nothing_is_built_until_assistive_technology_asks,
            ),
            (
                "the_sample_menu_reads_as_roles_names_values_and_states",
                the_sample_menu_reads_as_roles_names_values_and_states,
            ),
            (
                "pressing_a_button_through_the_system_clicks_it",
                pressing_a_button_through_the_system_clicks_it,
            ),
            (
                "focus_moves_both_ways_between_the_keyboard_and_the_system",
                focus_moves_both_ways_between_the_keyboard_and_the_system,
            ),
            (
                "a_slider_steps_and_takes_values_through_the_system",
                a_slider_steps_and_takes_values_through_the_system,
            ),
            (
                "a_toggle_flips_and_a_text_field_takes_text_through_the_system",
                a_toggle_flips_and_a_text_field_takes_text_through_the_system,
            ),
            (
                "renaming_disabling_hiding_and_describing_update_what_is_read",
                renaming_disabling_hiding_and_describing_update_what_is_read,
            ),
            (
                "a_modal_dialog_replaces_the_menu_and_closing_it_brings_the_menu_back",
                a_modal_dialog_replaces_the_menu_and_closing_it_brings_the_menu_back,
            ),
            (
                "an_unchanged_node_keeps_its_element_when_another_one_changes",
                an_unchanged_node_keeps_its_element_when_another_one_changes,
            ),
            (
                "a_request_for_a_node_that_is_gone_is_dropped",
                a_request_for_a_node_that_is_gone_is_dropped,
            ),
            (
                "the_example_menu_is_played_through_the_system_from_play_to_the_quit_dialog",
                the_example_menu_is_played_through_the_system_from_play_to_the_quit_dialog,
            ),
        ];
        // `cargo test <filter>` passes a filter; run the checks it matches.
        let filter = std::env::args().skip(1).find(|a| !a.starts_with('-'));
        println!("\nrunning {} checks on the main thread", checks.len());
        let mut ran = 0;
        for (name, check) in checks {
            if filter.as_ref().is_some_and(|f| !name.contains(f.as_str())) {
                continue;
            }
            // A failed check panics: the program exits non-zero and cargo
            // reports the test as failed, with the name printed last.
            print!("check {name} ... ");
            check();
            println!("ok");
            ran += 1;
        }
        println!("\ncheck result: ok. {ran} passed\n");
    }
}
