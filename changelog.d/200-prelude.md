### Added
- A curated `prelude`, public service modules, associated rendering/spatial constants and explicit raw ECS views.

### Changed
- `World`'s dynamic methods (`get_dynamic`, `insert_dynamic`, `spawn_dynamic`, `remove_dynamic`, the `*_resource_dynamic` calls, `for_each_chunk`, `component_by_name`, `archetypes`) are reached through `world.raw()` / `world.raw_ref()`; the `World` spellings are deprecated forwarders. Root constants and the remaining moved root types are deprecated in favour of their module or associated spellings.
- Specialized crate-root type spellings are deprecated compatibility aliases; they remain source-compatible, including public tuple constructors. The rustdoc landing page shows modules, with examples and guide code using canonical imports.
