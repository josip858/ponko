# Parallel queries across many small archetypes (#141)

Run `cargo run --release -p ponko_bench --bin ponko_bench -- par-arch`
(`src/par_archetypes.rs`). Every workload has `L` matching archetypes of
`R` entities each (`LxR`; `+N` adds one more archetype of `N` entities),
all `(Pos, Vel)` plus a different subset of zero-sized markers. *Light* is
`pos += vel * dt`; *heavy* is four `sin_cos` per entity (about 25 ns).
`seq` is `for_each` on one core, `par` is `par_for_each`, `collect` is
`par_collect_into` into a reused `Vec`. Steady benchmarks: 5 warmup runs,
then 200 timed runs (50 from 500k entities up).

## What changed

Before, `par_for_each_indexed` (and so `par_for_each` and
`par_collect_into`) cut each archetype into chunks and waited for it before
starting the next one, and ran every archetype of at most 512 rows on the
calling thread. Many small archetypes therefore used one core, and a few big
ones paid one parallel join per archetype.

Now the calling thread lines up the matching rows of all archetypes as
(archetype, row range, first query position) entries, stamping change ticks
and applying `Added`/`Changed` row filters as before, and one parallel pass
runs fixed-size chunks of the positions: a chunk can hold many small
archetypes or part of a big one. Positions, and the output order of
`par_collect_into`, are exactly as before. The plan's storage is owned by the
world and reused, so warm parallel queries allocate nothing
(`tests/parallel_alloc.rs`).

Thresholds:

| Case | Behavior |
|---|---|
| total rows ≤ one chunk (`clamp(total / (4 × threads), 512, 8192)`) | calling thread (unchanged) |
| some archetype has more than 512 matching rows | parallel plan |
| all archetypes ≤ 512 rows, more than one chunk | first 512 rows run on the calling thread, timed; if the whole query projects under **50 µs**, the rest runs there too (as before), otherwise in parallel |
| the same on the web (no clock) | calling thread (unchanged) |

## Measurements (2026-11-10)

Apple M4 Max (14 cores, 14 rayon threads), macOS, aarch64-apple-darwin,
rustc 1.99.1 (58cf8f9c3), Cargo release profile, default benchmark
features. *Old* is `929d125` (main plus this benchmark), *new* is the
branch head. Both binaries were built back to back and run alternately,
three rounds each, so both saw the same machine load. Values are the best
median (p50) of the three rounds, in ms. GPU metrics are not applicable.

Other builds were running throughout: the load average was 13–24 in the
first paired run and 37–78 in the second. Waking rayon's threads takes
longer under load, so the parallel times of small queries (≈ 0.01 ms) are
noisy; treat sub-0.05 ms parallel cells as indicative.

### Final code (second paired run, load 37–78)

| Workload | seq | old par | new par | new vs old |
|---|---:|---:|---:|---:|
| 1x1000000 light | 0.752 | 0.125 | 0.126 | 0.99× |
| 1x1000000 heavy | 25.1 | 3.01 | 2.87 | 1.05× |
| 4x250000 light | 0.755 | 0.266 | 0.128 | **2.1×** |
| 4x250000 heavy | 26.6 | 3.37 | 2.98 | 1.13× |
| 4096x256 light | 0.870 | 0.896 | 0.167 | **5.4×** |
| 4096x256 heavy | 30.0 | 29.5 | 3.55 | **8.3×** |
| 4096x16 light | 0.061 | 0.065 | 0.063 | 1.04× |
| 4096x16 heavy | 1.66 | 1.66 | 0.302 | **5.5×** |
| 1024x64 heavy | 1.65 | 1.64 | 0.303 | **5.4×** |
| 2048x16 light | 0.030 | 0.033 | 0.034 | 0.96× |
| 2048x16 heavy | 0.830 | 0.804 | 0.174 | **4.6×** |
| 1024x16 light | 0.015 | 0.016 | 0.017 | 0.92× |
| 1024x16 heavy | 0.417 | 0.407 | 0.098 | **4.1×** |
| 512x16 light | 0.0076 | 0.0082 | 0.0086 | 0.95× |
| 512x16 heavy | 0.201 | 0.198 | 0.091 | **2.2×** |
| 256x16 heavy | 0.106 | 0.103 | 0.095 | 1.09× (min 0.093 → 0.046) |
| 64x8 light / heavy | 0.0006 / 0.0115 | 0.0007 / 0.0114 | 0.0008 / 0.0117 | one chunk: unchanged path |
| 1024x64+500000 light | 0.427 | 0.147 | 0.102 | **1.45×** |
| 1024x64+500000 heavy | 14.5 | 3.03 | 1.62 | **1.87×** |
| `par_collect_into` 4096x256 light | – | 0.639 | 0.137 | **4.7×** |
| `par_collect_into` 4x250000 light | – | 0.205 | 0.099 | **2.1×** |
| cached query 4096x16 light | – | 0.048 | 0.034 | 1.4× |

### Single-archetype workloads (first paired run, load 13–24)

One archetype takes the same path in the first and the final build (the
timed first chunk applies only when every archetype is small), so this
quieter run is the regression check for homogeneous worlds:

| Workload | seq | old par | new par | new vs old |
|---|---:|---:|---:|---:|
| 1x1000000 light | 0.743 | 0.118 | 0.114 | 1.04× |
| 1x1000000 heavy | 25.4 | 2.86 | 2.67 | 1.07× |
| 1x1000000 collect | – | 0.109 | 0.086 | 1.27× |
| 1x65536 light | 0.049 | 0.049 | 0.050 | 0.98× |
| 1x65536 heavy | 1.61 | 0.272 | 0.276 | 0.98× |
| 1x16384 light | 0.012 | 0.017 | 0.017 | 0.98× |
| 1x16384 heavy | 0.406 | 0.084 | 0.085 | 0.99× |
| 1x4096 light | 0.0031 | 0.0065 | 0.0064 | 1.01× |
| 1x4096 heavy | 0.101 | 0.058 | 0.073 | 0.80× (min 0.028 → 0.031) |
| 1x1024 heavy | 0.023 | 0.031 | 0.027 | 1.15× |

All within run-to-run noise: no regression for homogeneous worlds. In the
second (loaded) run the same cells varied between 0.44× and 1.4× in both
directions, which is noise from thread wake-up under load, not the code.

### The first build, without the timed fallback

The first paired run measured the plan running every query over more than
4096 rows of small archetypes in parallel. Cheap work lost there:

| Workload | old par | plan, always parallel |
|---|---:|---:|
| 512x16 light (8k rows) | 0.0079 | 0.0192 (0.41×) |
| 1024x16 light (16k) | 0.0160 | 0.0442 (0.36×) |
| 2048x16 light (32k) | 0.0320 | 0.0460 (0.70×) |
| 4096x16 light (64k) | 0.0640 | 0.0620 (1.03×) |

while heavy work at the same sizes already ran 2.4–5.7× faster. The
crossover is a matter of time, not rows: it sits where the serial loop takes
about 50–60 µs (4096x16 light, 1x1024–1x4096 heavy). That is the budget the
timed first chunk compares against.

## Crossovers and tradeoffs

- **Many small archetypes:** parallel pays off once the loop takes more than
  about 50 µs on one core. Below that the query stays on the calling thread,
  as before. Heavy work crosses over at a few thousand rows (256x16 heavy,
  100 µs serial, runs about 2× faster at its best run); light work at about
  64k rows.
- **Cost of the fallback:** cheap queries over small archetypes that stay on
  the calling thread take 4–8% longer than before (about 0.3–0.9 µs at
  8k–32k rows): building the plan reads every archetype once more, and the
  first chunk is timed. Queries that fit one chunk skip the timing.
- **Timing makes the choice of thread, not the result:** positions, output
  order and change stamps are the same either way. A query slowed down by
  load during its timed first chunk may run the rest in parallel.
- **Single archetypes:** unchanged. As before, `par_for_each` over one archetype
  goes parallel from 513 rows, which costs a few microseconds for very cheap
  work (1x4096 light: 0.003 ms serial vs 0.006 ms parallel); `for_each`
  stays the right choice there.
- **The web:** `std::time::Instant` isn't available on `wasm32-unknown-unknown`,
  so queries over small archetypes keep running on the calling thread there.

## Correctness

`crates/ponko_ecs/tests/parallel.rs` checks positions, `par_for_each`
visits and `par_collect_into` output (plain and owning results) against
`iter()` order, with cheap and with slow per-entity work, for many tiny, a
few huge, mixed and empty archetypes, `with`/`without`/`Or` filters,
required and optional components, the cached query path, `Changed` runs
across many archetypes, mutable loops, and mapper panics in the timed first
chunk and in the parallel part. `tests/parallel_alloc.rs` checks that warm
parallel queries don't allocate on the querying thread. The unsafe code
passes under Miri (`parallel` and `changes` with tree borrows, and every
test without the `parallel` feature).
