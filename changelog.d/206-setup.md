### Changed
- The examples and the guide put startup code on the builder:
  `.setup(|ctx| ...)` followed by `.run()` or `.run_headless(ticks)`.
  Frame closures returned from setup became frame systems that own their
  state (`ants`, `boids`, `ecs`, `web_smoke`) or read a resource made at
  startup (`battle`, `flow`). `run(|ctx| ...)` stays for the one-closure
  programs (`window`, `sprites`, `bunnymark`) and
  `run_headless(ticks, |ctx| ...)` for a setup that borrows a local.
- The guide's "Getting started" has a "Setup on the builder" section, with
  the C++ `game::setup` next to it; `Game::run` and `Game::run_headless`
  docs say when to use `Game::setup` instead.
