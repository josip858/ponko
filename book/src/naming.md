# Ponko naming and compatibility

Cargo packages and source directories use `ponko` and `ponko_*`, starting at
version `0.0.1`. This naming change preserves the compatibility commitments in
[STABILITY.md](https://github.com/ponkoengine/ponko/blob/main/STABILITY.md).

## Native code

New C programs include `ponko.h` and use `Ponko*` types, `PONKO_*` constants
and `ponko_*` functions. New C++ programs include `ponko.hpp` and use `ponko::`.
`PONKO_PROFILE_SCOPE` and `PONKO_COUNT_CPP_ALLOCATIONS` provide the profiling
macros. Link against `ponko_ffi` as before.

These headers provide **source aliases**, including for function pointers, over
the existing ABI. The exported binary symbols remain `crabby_*`, the ABI version
remains 2, and layouts and calling conventions are unchanged. Dynamic library
loaders must keep looking up the existing symbols; independently exported
`ponko_*` symbols would be a separate additive ABI project. The `crabby.h` and
`crabby.hpp` headers, `crabby::` namespace, native types, constants and macros
continue to work. Both header spellings may be included in the same translation
unit. Rust users of `ponko_ffi` retain its existing low-level names.

## Environment settings

All engine runtime settings now accept `PONKO_*`: control, start-paused,
instance directory, launch ID, inspector, hot reload, HUD, vsync, benchmark and
trace settings (including their warmup, output and frame-count variants).
The corresponding `CRABBY_*` spelling remains a fallback. **Presence of the
Ponko spelling wins**, even if its value is empty, invalid, or `0`; fallback
happens only when it is absent. Parsing and defaults otherwise stay unchanged.
This also applies to the MCP server's instance-directory setting. MCP launches
set both spellings so older engine executables can still be launched.

For example, `PONKO_CONTROL=0 CRABBY_CONTROL=1` leaves the endpoint disabled.
`PONKO_BENCH=600` is the primary benchmark spelling. The CMake examples accept
`PONKO_LIBRARY_DIR` and `PONKO_INCLUDE_DIR`, with the old cache names as fallbacks.
`Color::PONKO_DARK` is the primary Rust color constant; `Color::CRABBY_DARK`
is an alias with the same value.

## Persistent and external names

These identifiers deliberately retain their old spelling:

- Saved worlds and prefabs use `crabby-world` and `crabby-prefab` with their
  existing versions. Existing saves and importer output remain interoperable.
- glTF extras retain `crabbyMorphFrames` and `crabbyAnimatedTargets`.
- The default instance registry remains `~/.crabby/instances` (or
  `%LOCALAPPDATA%\crabby\instances`), shared with existing games and tools.
  Set `PONKO_INSTANCES_DIR` to select another location. MCP launch logs keep
  their existing home directory, too.
- The web canvas ID remains `crabby` so existing embedding pages work.
- MCP tool names, parameters, protocol versions, control messages and benchmark
  JSON identifiers retain their established contracts. The reference benchmark
  app bundle keeps its existing name so previously documented commands work.
- Historical release notes, license notices and pre-existing README files retain
  their original text. The leak-check configuration path stays unchanged.

Changing persistent identifiers or removing aliases requires a separately
approved migration under the stability policy.

## Inventory and regression check

`naming-inventory.json` at the repository root inventories every tracked file
with case-insensitive legacy references, including generated native headers,
fixtures, metadata, historical text and paths. Each entry records its category,
reason and maximum occurrence count. `scripts/check-naming.py` rejects new
legacy-bearing files or paths and increased legacy counts; shrinking the
inventory is allowed. Review inventory changes alongside naming changes.
Generated aliases are checked separately by `scripts/generate-ponko-header.py
--check`. CI runs both checks. The Pages workflow builds this guide directly
from `book/` after changes reach `main`.

## Dimension modules

Start with `use ponko::prelude::*;`, then `use ponko::d2::*;` or
`use ponko::d3::*;`. Both expose `Transform`, `Camera`, `RigidBody`, `Collider`,
`Velocity`, `Shape`, `Physics`, and `Hit` (physics features required).
The modules are aliases for existing types: layout, component identity and saved
names are unchanged. Context fields remain `camera`/`camera3d` and
`physics`/`physics3d` so both dimensions can coexist.

| Retained root spelling | Preferred spelling |
|---|---|
| `Transform2D`, `Camera2D` | `d2::Transform`, `d2::Camera` |
| `RigidBody`, `Collider`, `Velocity`, `Shape`, `Physics`, `RayHit` | corresponding `d2` name (`Hit` for `RayHit`) |
| `AudioListener3D` | `d3::AudioListener` |
| `AudioSettings3D` | `d3::AudioSettings` |
| `AudioSource3D` | `d3::AudioSource` |
| `Camera3D` | `d3::Camera` |
| `Character3D` | `d3::Character` |
| `Collider3D` | `d3::Collider` |
| `ContactModifier3D` | `d3::ContactModifier` |
| `ContactPair3D` | `d3::ContactPair` |
| `Decal3D` | `d3::Decal` |
| `Hit3D` | `d3::Hit` |
| `Joint3D` | `d3::Joint` |
| `JointAxis3D` | `d3::JointAxis` |
| `JointDrive3D` | `d3::JointDrive` |
| `JointMotion3D` | `d3::JointMotion` |
| `Line3D` | `d3::Line` |
| `NavAgent3D` | `d3::NavAgent` |
| `NavMesh3D` | `d3::NavMesh` |
| `NavObstacle3D` | `d3::NavObstacle` |
| `ParticleBlend3D` | `d3::ParticleBlend` |
| `ParticleEmitter3D` | `d3::ParticleEmitter` |
| `ParticleSettings3D` | `d3::ParticleSettings` |
| `ParticleShape3D` | `d3::ParticleShape` |
| `ParticleSpace3D` | `d3::ParticleSpace` |
| `Physics3D` | `d3::Physics` |
| `PointLight3D` | `d3::PointLight` |
| `QueryFilter3D` | `d3::QueryFilter` |
| `Ragdoll3D` | `d3::Ragdoll` |
| `ReflectionProbe3D` | `d3::ReflectionProbe` |
| `RigidBody3D` | `d3::RigidBody` |
| `Shape3D` | `d3::Shape` |
| `SpotLight3D` | `d3::SpotLight` |
| `Text3D` | `d3::Text` |
| `TextAlign3D` | `d3::TextAlign` |
| `Trail3D` | `d3::Trail` |
| `Transform3D` | `d3::Transform` |
| `UiPanel3D` | `d3::UiPanel` |
| `Vehicle3D` | `d3::Vehicle` |
| `Velocity3D` | `d3::Velocity` |
| `Wheel3D` | `d3::Wheel` |

Use `physics.cast_ray_filtered(origin, direction, max_distance, filter)` in
both dimensions and `physics.overlap_ball(center, radius, filter, &mut hits)`
for circles/spheres. Filters default to solid colliders, can include sensors,
and can exclude an entity. Both `Hit` types carry `entity`, `point`, `normal`
and `distance`; vectors have the matching dimension. `apply_impulse` and `mass`
share the same convention. Existing `cast_ray`, `entities_at`,
`entities_in_circle` and `overlap_sphere` calls retain their behavior.
The issue sketch called the common method `cast_ray`; Rust cannot overload
inherent methods by argument count, so the additive filtered spelling preserves
the existing three-argument 2D call and four-argument 3D call.

`Ctx::spawn_prefab(path, placement)` also accepts a `Vec3` or `d3::Transform`.
A 3D transform supplies position, rotation and scale; a 2D position keeps the
prefab's existing rotation and scale. Saved component names remain unchanged.
