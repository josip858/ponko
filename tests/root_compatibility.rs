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
    Game::new("legacy materials", 1, 1).run_headless(0, |ctx| {
        let material = ctx.gfx.default_material();
        let ModelMaterial(actual) = ModelMaterial(material);
        assert_eq!(actual, material);
    });
    let _: render::LodHysteresis = LodHysteresis(0.5);
    assert_eq!(std::mem::size_of::<Transform3D>(), std::mem::size_of::<d3::Transform>());
    assert_eq!(std::mem::align_of::<Transform3D>(), 16);
    let _: d3::ReceivesDecals = ReceivesDecals(1);
    let LodHysteresis(value) = LodHysteresis(0.5);
    let ReceivesDecals(mask) = ReceivesDecals(1);
    assert_eq!((value, mask), (0.5, 1));
}
#[cfg(feature = "physics3d")]
#[test]
fn old_enum_imports_remain_available() {
    use ponko::Shape3D::Sphere;
    assert!(matches!(Sphere { radius: 1.0 }, d3::Shape::Sphere { radius: 1.0 }));
}

#[test]
fn old_root_service_types_do_not_require_an_optional_subsystem() {
    // This is the old default import contract, independently of the new prelude.
    // In particular, turning 3D off must not hide 2D, UI, networking or profiling.
    macro_rules! available { ($($ty:ty),* $(,)?) => { $(let _: Option<$ty> = None;)* }; }
    available!(
        SystemAccess,
        Service,
        AccessConflict,
        Audio,
        AudioEffects,
        Camera2D,
        Transform2D,
        Sprite,
        Assets,
        MemorySource,
        AssetInventory,
        AssetClass,
        GpuResidency,
        Input,
        Sound,
        SoundParams,
        Playing,
        Font,
        TextParams,
        Texture,
        DrawParams,
        Rect,
        Graphics,
        Time,
        Turns,
        ActionMap,
        InputEvent,
        Random,
        Rng,
        Net,
        NetState,
        Pose,
        Authority,
        Interest,
        Perf,
        PerfReport,
        Budget,
        AllocReport,
        ScopeTiming,
        FrameStats,
        SystemTiming,
        Ui,
        UiId,
        UiImmediate,
        UiInput,
        UiLayout,
        UiPaint,
        UiRole,
        UiStyle,
        UiTextAlign,
        UiTheme,
        UiWidget,
        UiAccessNode,
        UiAccessTree,
        UiAccessValue,
    );
    let _: fn() -> perf::AllocCounts = alloc_counts;
    let _: fn(Vec2, f32) -> u64 = spatial_key;
    const DIFFERENCE: i16 = seq_diff(1, 0);
    assert_eq!(DIFFERENCE, 1);
}
