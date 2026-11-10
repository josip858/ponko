### Changed
- `Query::par_for_each`, `par_for_each_indexed` and `par_collect_into` run
  the matching rows of all archetypes in one parallel pass, instead of
  chunking each archetype and waiting for it before the next. Queries over
  many small archetypes now use every core (4096 archetypes of 256 entities
  with heavy per-entity work: about 8× faster on 14 cores), and a few big
  archetypes no longer wait for each other (up to 2× on cheap work).
  Positions and output order are unchanged, and the work plan's storage is
  reused, so warm parallel queries don't allocate. A query whose archetypes
  are all small times its first 512 rows and stays on the calling thread
  when the whole loop would take under 50 µs, as such queries did before.
  Measurements: `crates/ponko_bench/PAR_ARCHETYPES.md`.
