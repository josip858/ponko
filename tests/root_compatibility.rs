//! Intentional coverage of retained root spellings; application code uses modules.
#![allow(deprecated)]
use ponko::*;
#[test]
fn old_glob_and_public_constructors_keep_their_identity() {
    let _: d2::Transform = Transform2D::at(vec2(1.0, 2.0));
    let mut world = World::new();
    let entity = world.spawn((Transform2D::default(),));
    assert!(world.get::<d2::Transform>(entity).is_some());
    let _: UiId = Ui::default().root().id();
}
#[cfg(feature = "3d")]
#[test]
fn old_3d_glob_preserves_component_layout_and_tuple_construction() {
    let _: d3::Transform = Transform3D::default();
    let _: render::LodHysteresis = LodHysteresis(0.5);
    assert_eq!(std::mem::size_of::<Transform3D>(), std::mem::size_of::<d3::Transform>());
    assert_eq!(std::mem::align_of::<Transform3D>(), 16);
    let _: d3::ReceivesDecals = ReceivesDecals(1);
}
#[cfg(feature = "physics3d")]
#[test]
fn old_enum_imports_remain_available() {
    use ponko::Shape3D::Sphere;
    assert!(matches!(Sphere { radius: 1.0 }, d3::Shape::Sphere { radius: 1.0 }));
}
