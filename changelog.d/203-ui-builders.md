### Added
- Retained UI titles: `Ui::title(parent, text)` and `UiTheme::title` /
  `title_size`; titles follow the theme like other nodes.
- Nodes found by name: `UiBuilder::name`, `Ui::set_name`, `Ui::find`,
  `Ui::name`, so update systems need no resource of ids. `Ui::take_click`
  returns the last clicked node for screens that only react to clicks.
- `UiLayout::top` and `UiLayout::left` place a preset in points (a centered
  dialog near the top).
- C and C++: layout presets (`ponko_ui_layout_row`, `_column`, `_fill`,
  `_fixed`, `_anchored`, `_centered`; C++ `ui::row`, `ui::fill`, ... with
  `.top` / `.left`), the theme (`PonkoUiTheme`, `ponko_ui_theme_default`,
  `ponko_ctx_ui_theme`, `ponko_ctx_ui_set_theme`), titles
  (`ponko_ctx_ui_add_title`), names (`ponko_ctx_ui_set_name`, `_find`,
  `_name`), `ponko_ctx_ui_take_click` and `ponko_ctx_ui_paint`, and the matching C++
  `ui::tree` methods.

### Changed
- One theme for the screen's UI: `Ui::set_theme` also restyles the
  immediate widgets (`ctx.ui.immediate.style`), keeping fields set there by
  hand; `UiTheme::style()` is the style it gives them.
- The `ui_shop` and `ui_panel` examples are built with the builders, presets
  and names, at half their old length; an offscreen GPU test checks they draw
  the same pixels as the hand-built trees they replace.

### Fixed
- A screen root's `auto` margins now take the window's free space (both
  sides center it), as they do for a child; before, they were ignored.
