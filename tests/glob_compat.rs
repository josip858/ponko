//! `use ponko::*;` next to a module glob: every name both bring in is the
//! same item, so neither glob makes it ambiguous. Each `use self::Name as _;`
//! fails to compile (E0659) if the two globs disagree on `Name`. The names that
//! differ by dimension on purpose are listed in book/src/naming.md.
#![allow(deprecated, unused_imports)]

mod root_and_d2 {
    use self::Animation as _;
    use self::Attached as _;
    #[cfg(any(feature = "physics", feature = "physics3d"))]
    use self::BodyKind as _;
    use self::Camera as _;
    #[cfg(feature = "physics")]
    use self::Collider as _;
    #[cfg(any(feature = "physics", feature = "physics3d"))]
    use self::CollisionStarted as _;
    #[cfg(any(feature = "physics", feature = "physics3d"))]
    use self::CollisionStopped as _;
    use self::FlowField as _;
    #[cfg(feature = "physics")]
    use self::Hit as _;
    use self::NavGrid as _;
    #[cfg(feature = "physics")]
    use self::Physics as _;
    #[cfg(feature = "physics")]
    use self::RigidBody as _;
    #[cfg(feature = "physics")]
    use self::Shape as _;
    use self::SpatialGrid as _;
    use self::Sprite as _;
    use self::SpriteSheet as _;
    use self::TileMap as _;
    use self::Transform as _;
    #[cfg(feature = "physics")]
    use self::Velocity as _;
    use ponko::d2::*;
    use ponko::*;
    // Different items on purpose: QueryFilter.
}

#[cfg(feature = "3d")]
mod root_and_d3 {
    use self::AmbientOcclusion as _;
    use self::AnimationClip as _;
    use self::AnimationCondition as _;
    use self::AnimationEvent as _;
    use self::AnimationGraph as _;
    use self::AnimationLayer as _;
    use self::AnimationParameter as _;
    use self::AnimationState as _;
    use self::AnimationTrack as _;
    use self::AnimationTransition as _;
    use self::Animator as _;
    use self::AntiAliasing as _;
    use self::AttachToJoint as _;
    use self::AudioListener as _;
    use self::AudioSettings as _;
    use self::AudioSource as _;
    use self::Blend as _;
    #[cfg(any(feature = "physics", feature = "physics3d"))]
    use self::BodyKind as _;
    use self::Camera as _;
    #[cfg(feature = "physics3d")]
    use self::Character as _;
    #[cfg(feature = "physics3d")]
    use self::ColliderFocus as _;
    #[cfg(feature = "physics3d")]
    use self::CollisionGroups as _;
    #[cfg(feature = "physics3d")]
    use self::CollisionMesh as _;
    #[cfg(feature = "physics3d")]
    use self::CollisionMeshData as _;
    #[cfg(any(feature = "physics", feature = "physics3d"))]
    use self::CollisionStarted as _;
    #[cfg(any(feature = "physics", feature = "physics3d"))]
    use self::CollisionStopped as _;
    use self::Compute as _;
    use self::ComputePass as _;
    #[cfg(feature = "physics3d")]
    use self::ContactModifier as _;
    #[cfg(feature = "physics3d")]
    use self::ContactPair as _;
    use self::Decal as _;
    use self::Effect as _;
    use self::FarTerrain as _;
    #[cfg(feature = "physics3d")]
    use self::FrictionCurve as _;
    use self::GlobalIllumination as _;
    use self::GpuBuffer as _;
    use self::GraphicsQuality as _;
    use self::Heightfield as _;
    #[cfg(feature = "physics3d")]
    use self::Hit as _;
    use self::InstanceBatch as _;
    use self::InstanceBatchOptions as _;
    use self::InstanceLevel as _;
    use self::Interpolation as _;
    use self::JointAnimation as _;
    #[cfg(feature = "physics3d")]
    use self::JointAxis as _;
    #[cfg(feature = "physics3d")]
    use self::JointDrive as _;
    use self::JointMask as _;
    #[cfg(feature = "physics3d")]
    use self::JointMotion as _;
    use self::LayerBlend as _;
    use self::Light as _;
    use self::Lighting as _;
    use self::Line as _;
    use self::LodGroup as _;
    use self::LodHysteresis as _;
    use self::LodLevel as _;
    use self::LodPick as _;
    use self::LodRenderer as _;
    use self::LodSettings as _;
    use self::LodShadow as _;
    use self::Material as _;
    use self::MaterialDesc as _;
    use self::Mesh as _;
    use self::MeshCost as _;
    use self::MeshData as _;
    use self::Model as _;
    use self::ModelMaterial as _;
    use self::ModelMaterials as _;
    use self::MorphAnimation as _;
    use self::MorphFrame as _;
    use self::MorphTarget as _;
    use self::MorphWeights as _;
    use self::Motion as _;
    use self::MotionChild as _;
    #[cfg(feature = "navmesh")]
    use self::NavAgent as _;
    #[cfg(feature = "navmesh")]
    use self::NavMesh as _;
    #[cfg(feature = "navmesh")]
    use self::NavObstacle as _;
    use self::ParticleBlend as _;
    use self::ParticleCurve as _;
    use self::ParticleEmitter as _;
    use self::ParticleGradient as _;
    use self::ParticleSettings as _;
    use self::ParticleShape as _;
    use self::ParticleSpace as _;
    use self::PointLight as _;
    use self::Post as _;
    use self::QualityLevel as _;
    #[cfg(feature = "physics3d")]
    use self::Ragdoll as _;
    #[cfg(feature = "physics3d")]
    use self::RagdollBone as _;
    use self::ReceivesDecals as _;
    use self::ReflectionProbe as _;
    use self::RootMotion as _;
    use self::ScreenEffect as _;
    use self::Shader as _;
    use self::ShaderDiagnostic as _;
    use self::ShaderKind as _;
    use self::ShaderOutcome as _;
    use self::ShaderStatus as _;
    use self::ShadowMesh as _;
    use self::Skeleton as _;
    use self::SkinOf as _;
    use self::Sky as _;
    use self::SpotLight as _;
    use self::StreamedLod as _;
    use self::TargetCamera as _;
    use self::TargetEffect as _;
    #[cfg(feature = "physics3d")]
    use self::TerrainCollider as _;
    use self::TerrainMaterialDesc as _;
    use self::TerrainTile as _;
    use self::TerrainTileData as _;
    use self::Text as _;
    use self::TextAlign as _;
    use self::TextureArray as _;
    use self::TextureKind as _;
    use self::Tonemap as _;
    use self::Trail as _;
    use self::Transform as _;
    use self::UiPanel as _;
    #[cfg(feature = "physics3d")]
    use self::Vehicle as _;
    #[cfg(feature = "physics3d")]
    use self::Wheel as _;
    use ponko::d3::*;
    use ponko::*;
    // Different items on purpose: Collider, RigidBody, Velocity, Shape, Physics, QueryFilter, Joint.
}

mod root_and_prelude {
    use self::Action as _;
    use self::ActionId as _;
    use self::ActionMap as _;
    use self::ActionRef as _;
    use self::Added as _;
    use self::AssetSource as _;
    use self::Assets as _;
    use self::AudioBus as _;
    use self::Axis2Binding as _;
    use self::AxisBinding as _;
    #[cfg(any(feature = "physics", feature = "physics3d"))]
    use self::BodyKind as _;
    use self::ButtonBinding as _;
    use self::Camera2D as _;
    #[cfg(feature = "3d")]
    use self::Camera3D as _;
    use self::Changed as _;
    #[cfg(any(feature = "physics", feature = "physics3d"))]
    use self::CollisionStarted as _;
    #[cfg(any(feature = "physics", feature = "physics3d"))]
    use self::CollisionStopped as _;
    use self::Color as _;
    use self::Commands as _;
    use self::Component as _;
    use self::ConfiguredGame as _;
    use self::Ctx as _;
    use self::DrawParams as _;
    use self::Entity as _;
    use self::Error as _;
    use self::ErrorKind as _;
    use self::Events as _;
    use self::Font as _;
    use self::Game as _;
    use self::Handle as _;
    use self::Key as _;
    #[cfg(feature = "3d")]
    use self::Mat4 as _;
    use self::MemorySource as _;
    #[cfg(feature = "3d")]
    use self::Model as _;
    use self::MouseButton as _;
    use self::Or as _;
    use self::Phase as _;
    use self::Placement as _;
    use self::Playing as _;
    use self::Plugin as _;
    use self::Quat as _;
    use self::Query as _;
    use self::Random as _;
    use self::Rect as _;
    use self::Rng as _;
    use self::SetupResult as _;
    use self::Sound as _;
    use self::SoundParams as _;
    use self::Sprite as _;
    use self::SpriteSheet as _;
    use self::SystemId as _;
    use self::Systems as _;
    use self::TextParams as _;
    use self::Texture as _;
    use self::TextureFilter as _;
    use self::Tick as _;
    use self::Time as _;
    use self::Transform2D as _;
    #[cfg(feature = "3d")]
    use self::Transform3D as _;
    use self::Turns as _;
    use self::UVec2 as _;
    use self::Ui as _;
    use self::UiId as _;
    use self::Vec2 as _;
    use self::Vec3 as _;
    #[cfg(feature = "3d")]
    use self::Vec4 as _;
    use self::With as _;
    use self::Without as _;
    use self::World as _;
    use self::WorldReflect as _;
    use self::actions as _;
    use self::d2 as _;
    #[cfg(feature = "3d")]
    use self::d3 as _;
    use self::json as _;
    use self::profile_scope as _;
    use self::systems as _;
    use self::uvec2 as _;
    use self::vec2 as _;
    use self::vec3 as _;
    #[cfg(feature = "3d")]
    use self::vec4 as _;
    use ponko::prelude::*;
    use ponko::*;
}

/// Every prelude item is also at the root, as the same item (the root glob
/// and the prelude glob above agree), and none of them is deprecated there.
#[deny(deprecated)]
mod root_holds_every_prelude_item {
    use ponko::Action as _;
    use ponko::ActionId as _;
    use ponko::ActionMap as _;
    use ponko::ActionRef as _;
    use ponko::Added as _;
    use ponko::AssetSource as _;
    use ponko::Assets as _;
    use ponko::AudioBus as _;
    use ponko::Axis2Binding as _;
    use ponko::AxisBinding as _;
    #[cfg(any(feature = "physics", feature = "physics3d"))]
    use ponko::BodyKind as _;
    use ponko::ButtonBinding as _;
    use ponko::Camera2D as _;
    #[cfg(feature = "3d")]
    use ponko::Camera3D as _;
    use ponko::Changed as _;
    #[cfg(any(feature = "physics", feature = "physics3d"))]
    use ponko::CollisionStarted as _;
    #[cfg(any(feature = "physics", feature = "physics3d"))]
    use ponko::CollisionStopped as _;
    use ponko::Color as _;
    use ponko::Commands as _;
    use ponko::Component as _;
    use ponko::ConfiguredGame as _;
    use ponko::Ctx as _;
    use ponko::DrawParams as _;
    use ponko::Entity as _;
    use ponko::Error as _;
    use ponko::ErrorKind as _;
    use ponko::Events as _;
    use ponko::Font as _;
    use ponko::Game as _;
    use ponko::Handle as _;
    use ponko::Key as _;
    #[cfg(feature = "3d")]
    use ponko::Mat4 as _;
    use ponko::MemorySource as _;
    #[cfg(feature = "3d")]
    use ponko::Model as _;
    use ponko::MouseButton as _;
    use ponko::Or as _;
    use ponko::Phase as _;
    use ponko::Placement as _;
    use ponko::Playing as _;
    use ponko::Plugin as _;
    use ponko::Quat as _;
    use ponko::Query as _;
    use ponko::Random as _;
    use ponko::Rect as _;
    use ponko::Rng as _;
    use ponko::SetupResult as _;
    use ponko::Sound as _;
    use ponko::SoundParams as _;
    use ponko::Sprite as _;
    use ponko::SpriteSheet as _;
    use ponko::SystemId as _;
    use ponko::Systems as _;
    use ponko::TextParams as _;
    use ponko::Texture as _;
    use ponko::TextureFilter as _;
    use ponko::Tick as _;
    use ponko::Time as _;
    use ponko::Transform2D as _;
    #[cfg(feature = "3d")]
    use ponko::Transform3D as _;
    use ponko::Turns as _;
    use ponko::UVec2 as _;
    use ponko::Ui as _;
    use ponko::UiId as _;
    use ponko::Vec2 as _;
    use ponko::Vec3 as _;
    #[cfg(feature = "3d")]
    use ponko::Vec4 as _;
    use ponko::With as _;
    use ponko::Without as _;
    use ponko::World as _;
    use ponko::WorldReflect as _;
    use ponko::actions as _;
    use ponko::d2 as _;
    #[cfg(feature = "3d")]
    use ponko::d3 as _;
    use ponko::json as _;
    use ponko::profile_scope as _;
    use ponko::systems as _;
    use ponko::uvec2 as _;
    use ponko::vec2 as _;
    use ponko::vec3 as _;
    #[cfg(feature = "3d")]
    use ponko::vec4 as _;
    #[allow(dead_code)]
    fn uses_the_names(
        _: Option<(
            ponko::Transform2D,
            ponko::Camera2D,
            ponko::Sprite,
            ponko::Ui,
            ponko::UiId,
        )>,
    ) {
    }
    #[cfg(feature = "3d")]
    #[allow(dead_code)]
    fn uses_the_3d_names(_: Option<(ponko::Transform3D, ponko::Camera3D, ponko::Model)>) {}
}

#[test]
fn the_root_glob_combines_with_the_prelude_and_dimension_globs() {
    // Compiling this file is the test.
}
