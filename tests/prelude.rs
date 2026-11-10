//! Importing the prelude must leave common game-owned names available.
#![deny(deprecated)]
use ponko::prelude::*;
struct Pose;
struct Material;
struct Job;
#[test]
fn common_game_names_do_not_collide_with_the_prelude() {
    let _ = (Pose, Material, Job);
    let mut world = World::new();
    let entity = world.spawn((Transform2D::at(vec2(1.0, 2.0)),));
    assert_eq!(world.get::<d2::Transform>(entity).unwrap().position, vec2(1.0, 2.0));
    let _ = Game::new("prelude", 1, 1).run_headless(0, |_| {});
}
#[cfg(feature = "3d")]
#[test]
fn constants_are_discovered_from_their_owning_types() {
    assert_eq!(d3::Decal::MAX, 4096);
    assert_eq!(ponko::spatial::SpatialGrid::<()>::SORT_EVERY, 32);
    assert_eq!(ponko::render::GraphicsQuality::MIN_RENDER_SCALE, 0.25);
    assert_eq!(ponko::terrain::TerrainTile::LODS, 4);
}
