//! Where the engine's own service steps sit in a windowed frame, driven
//! without a window: `begin_frame` then `run_systems`, as `window.rs` does.
//!
//! The defaults pinned here are part of the documented frame order in
//! ARCHITECTURE.md: retained UI updates before any simulation or frame
//! system, built-in animation and particles run between the frame systems
//! and the late systems, positional audio follows the late and drawing
//! systems. Headless runs skip all of these (`tests/service_phases.rs`).

use super::*;

#[derive(Default)]
struct Log(Vec<String>);

fn ctx() -> Ctx {
    let mut ctx = Ctx::new(1.0 / 60.0, Vec2::new(800.0, 600.0));
    // No device: positional audio hands out silent voices.
    ctx.set_headless(true);
    ctx.world.insert_resource(Log::default());
    ctx
}

fn note(ctx: &mut Ctx, line: impl Into<String>) {
    ctx.world.resource_mut::<Log>().0.push(line.into());
}

fn take(ctx: &mut Ctx) -> Vec<String> {
    std::mem::take(&mut ctx.world.resource_mut::<Log>().0)
}

/// A game without the built-in drawing systems.
fn bare(game: Game) -> Game {
    game.remove_system(animate_sprites)
        .remove_system(draw_tilemaps)
        .remove_system(draw_sprites)
}

/// One windowed frame running `ticks` simulation ticks, then the end-of-frame
/// input reset, as the window loop does.
fn frame(game: &mut Game, ctx: &mut Ctx, ticks: u32) {
    ctx.begin_frame(FrameTiming {
        dt: 1.0 / 60.0,
        fixed_steps: ticks,
        alpha: 0.0,
    });
    game.run_systems(ctx, ticks);
    ctx.input.end_frame();
}

#[test]
fn retained_ui_updates_before_simulation_and_frame_systems() {
    use crate::ui_layout::{Dimension, Size, Style};
    let mut ctx = ctx();
    let root = ctx.ui.add(
        crate::UiWidget::Panel,
        Style {
            size: Size {
                width: Dimension::length(300.0),
                height: Dimension::length(200.0),
            },
            ..Default::default()
        },
    );
    let button = ctx.ui.add_child(
        root,
        crate::UiWidget::Button,
        Style {
            size: Size {
                width: Dimension::length(70.0),
                height: Dimension::length(30.0),
            },
            flex_shrink: 0.0,
            ..Default::default()
        },
    );
    ctx.ui.push_screen(root, false);
    let mut game = bare(Game::new("t", 1, 1))
        .sim_system(|ctx: &mut Ctx| {
            let line = format!("sim focus={}", ctx.ui.focus().is_some());
            note(ctx, line);
        })
        .system(|ctx: &mut Ctx| {
            let line = format!("frame focus={}", ctx.ui.focus().is_some());
            note(ctx, line);
        });
    assert_eq!(ctx.ui.focus(), None);
    // Tab moves the focus to the button when the UI updates. The systems of
    // the same frame already see it: the UI went first.
    ctx.input.feed(crate::InputEvent::Key(crate::Key::Tab, true));
    frame(&mut game, &mut ctx, 1);
    assert_eq!(take(&mut ctx), ["sim focus=true", "frame focus=true"]);
    assert_eq!(ctx.ui.focus(), Some(button));
}

#[test]
fn ticks_per_frame_follow_the_requested_count() {
    let mut ctx = ctx();
    let mut game = bare(Game::new("t", 1, 1)).sim_system(|ctx: &mut Ctx| {
        let tick = ctx.time.sim_tick();
        note(ctx, format!("tick {tick}"));
    });
    // Zero ticks (a frame between ticks) and three ticks.
    frame(&mut game, &mut ctx, 0);
    frame(&mut game, &mut ctx, 3);
    assert_eq!(take(&mut ctx), ["tick 0", "tick 1", "tick 2"]);
}

#[cfg(feature = "3d")]
mod positional_audio {
    use super::*;
    use crate::{AudioSettings3D, AudioSource3D, Transform3D, Vec3};

    #[test]
    fn positional_audio_follows_late_and_draw_systems() {
        let mut ctx = ctx();
        let sound = ctx.audio.create(100, &[0.0; 100]);
        let moved = ctx.world.spawn((
            Transform3D::default(),
            AudioSource3D::new(sound, AudioSettings3D::default()),
        ));
        let mut game = bare(Game::new("t", 1, 1))
            .system(|ctx: &mut Ctx| {
                let silent = ctx.audio.backend.spatial.voice_positions().len();
                note(ctx, format!("frame voices={silent}"));
            })
            .late_system(move |ctx: &mut Ctx| {
                let started = ctx.world.get::<AudioSource3D>(moved).unwrap().started();
                note(ctx, format!("late started={started}"));
                ctx.world.get_mut::<Transform3D>(moved).unwrap().position = Vec3::X * 4.0;
            });
        game.draw_systems.push(System::new(move |ctx: &mut Ctx| {
            let started = ctx.world.get::<AudioSource3D>(moved).unwrap().started();
            note(ctx, format!("draw started={started}"));
            ctx.world.get_mut::<Transform3D>(moved).unwrap().position = Vec3::X * 9.0;
        }));
        frame(&mut game, &mut ctx, 1);
        assert_eq!(
            take(&mut ctx),
            ["frame voices=0", "late started=false", "draw started=false"],
            "no system sees a voice before the audio step"
        );
        assert_eq!(
            ctx.audio.backend.spatial.voice_positions(),
            [Vec3::X * 9.0],
            "the emitter started once the frame's systems were done, at its final place"
        );
    }

    #[test]
    fn emitters_spawned_by_late_systems_start_in_the_same_frame() {
        let mut ctx = ctx();
        let sound = ctx.audio.create(100, &[0.0; 100]);
        let mut game = bare(Game::new("t", 1, 1)).late_system(move |ctx: &mut Ctx| {
            ctx.commands.spawn((
                Transform3D::default(),
                AudioSource3D::new(sound, AudioSettings3D::default()),
            ));
        });
        frame(&mut game, &mut ctx, 0);
        assert_eq!(ctx.audio.backend.spatial.voice_positions().len(), 1);
        assert!(ctx.world.query::<&AudioSource3D>().iter().all(|s| s.started()));
    }

    #[test]
    fn the_listener_is_the_cameras_final_place_of_the_frame() {
        let mut ctx = ctx();
        let mut game = bare(Game::new("t", 1, 1)).late_system(|ctx: &mut Ctx| {
            ctx.camera3d.position = Vec3::new(1.0, 2.0, 3.0);
        });
        frame(&mut game, &mut ctx, 0);
        assert_eq!(ctx.audio.listener().position, Vec3::new(1.0, 2.0, 3.0));
    }
}

#[test]
fn a_frame_runs_ticks_then_frame_then_late_then_draw_and_applies_commands_between() {
    let mut ctx = ctx();
    let mut game = bare(Game::new("t", 1, 1))
        .sim_system(|ctx: &mut Ctx| note(ctx, "sim"))
        .add(Phase::Frame, |ctx| note(ctx, "frame"))
        .add(Phase::Animate, |ctx| note(ctx, "animate"))
        .late_system(|ctx: &mut Ctx| note(ctx, "late"));
    game.draw_systems.push(System::new(|ctx: &mut Ctx| note(ctx, "draw")));
    frame(&mut game, &mut ctx, 2);
    assert_eq!(take(&mut ctx), ["sim", "sim", "frame", "animate", "late", "draw"]);
}
