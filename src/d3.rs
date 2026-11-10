//! Three-dimensional game types. Pair with `ponko::prelude::*`.
pub use crate::audio::AudioListener3D as AudioListener;
pub use crate::audio::AudioSettings3D as AudioSettings;
pub use crate::audio::AudioSource3D as AudioSource;
#[cfg(feature = "navmesh")]
pub use crate::nav3d::NavAgent3D as NavAgent;
#[cfg(feature = "navmesh")]
pub use crate::nav3d::NavMesh3D as NavMesh;
#[cfg(feature = "navmesh")]
pub use crate::nav3d::NavObstacle3D as NavObstacle;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::Character3D as Character;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::Collider3D as Collider;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::ContactModifier3D as ContactModifier;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::ContactPair3D as ContactPair;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::Hit3D as Hit;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::Joint3D as Joint;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::JointAxis3D as JointAxis;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::JointDrive3D as JointDrive;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::JointMotion3D as JointMotion;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::Physics3D as Physics;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::QueryFilter3D as QueryFilter;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::Ragdoll3D as Ragdoll;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::RigidBody3D as RigidBody;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::Shape3D as Shape;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::Vehicle3D as Vehicle;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::Velocity3D as Velocity;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::Wheel3D as Wheel;
#[cfg(feature = "physics3d")]
pub use crate::physics3d::{
    ColliderFocus, CollisionGroups, CollisionMesh, CollisionMeshData, FrictionCurve, RagdollBone, TerrainCollider,
};
pub use crate::three::AmbientOcclusion;
pub use crate::three::AnimationClip;
pub use crate::three::AnimationCondition;
pub use crate::three::AnimationEvent;
pub use crate::three::AnimationGraph;
pub use crate::three::AnimationLayer;
pub use crate::three::AnimationParameter;
pub use crate::three::AnimationState;
pub use crate::three::AnimationTrack;
pub use crate::three::AnimationTransition;
pub use crate::three::Animator;
pub use crate::three::AntiAliasing;
pub use crate::three::AttachToJoint;
pub use crate::three::Blend;
pub use crate::three::Camera3D as Camera;
pub use crate::three::Compute;
pub use crate::three::ComputePass;
pub use crate::three::Decal3D as Decal;
pub use crate::three::Effect;
pub use crate::three::FarTerrain;
pub use crate::three::GlobalIllumination;
pub use crate::three::GpuBuffer;
pub use crate::three::GraphicsQuality;
pub use crate::three::Heightfield;
pub use crate::three::InstanceBatch;
pub use crate::three::InstanceBatchOptions;
pub use crate::three::InstanceLevel;
pub use crate::three::Interpolation;
pub use crate::three::JointAnimation;
pub use crate::three::JointMask;
pub use crate::three::LIGHT_SHADOW_ATLAS_SIZE;
pub use crate::three::LIGHT_SHADOW_SIZE;
pub use crate::three::LayerBlend;
pub use crate::three::Lighting;
pub use crate::three::Line3D as Line;
pub use crate::three::LodGroup;
pub use crate::three::LodHysteresis;
pub use crate::three::LodLevel;
pub use crate::three::LodPick;
pub use crate::three::LodRenderer;
pub use crate::three::LodSettings;
pub use crate::three::LodShadow;
pub use crate::three::MAX_DECALS;
pub use crate::three::MAX_DECALS_PER_CLUSTER;
pub use crate::three::MAX_GI_PROBES;
pub use crate::three::MAX_GI_PROBES_PER_FRAME;
pub use crate::three::MAX_LIGHT_SHADOW_VIEWS;
pub use crate::three::MAX_RENDER_SCALE;
pub use crate::three::MIN_RENDER_SCALE;
pub use crate::three::Material;
pub use crate::three::MaterialDesc;
pub use crate::three::Mesh;
pub use crate::three::MeshCost;
pub use crate::three::MeshData;
pub use crate::three::Model;
pub use crate::three::ModelMaterial;
pub use crate::three::ModelMaterials;
pub use crate::three::MorphAnimation;
pub use crate::three::MorphFrame;
pub use crate::three::MorphTarget;
pub use crate::three::MorphWeights;
pub use crate::three::Motion;
pub use crate::three::MotionChild;
pub use crate::three::ParticleBlend3D as ParticleBlend;
pub use crate::three::ParticleCurve;
pub use crate::three::ParticleEmitter3D as ParticleEmitter;
pub use crate::three::ParticleGradient;
pub use crate::three::ParticleSettings3D as ParticleSettings;
pub use crate::three::ParticleShape3D as ParticleShape;
pub use crate::three::ParticleSpace3D as ParticleSpace;
pub use crate::three::PointLight3D as PointLight;
pub use crate::three::PointLight3D as Light;
pub use crate::three::Post;
pub use crate::three::QualityLevel;
pub use crate::three::ReceivesDecals;
pub use crate::three::ReflectionProbe3D as ReflectionProbe;
pub use crate::three::RootMotion;
pub use crate::three::ScreenEffect;
pub use crate::three::Shader;
pub use crate::three::ShaderDiagnostic;
pub use crate::three::ShaderKind;
pub use crate::three::ShaderOutcome;
pub use crate::three::ShaderStatus;
pub use crate::three::ShadowMesh;
pub use crate::three::Skeleton;
pub use crate::three::SkinOf;
pub use crate::three::Sky;
pub use crate::three::SpotLight3D as SpotLight;
pub use crate::three::TERRAIN_LAYERS;
pub use crate::three::TERRAIN_LODS;
pub use crate::three::TargetCamera;
pub use crate::three::TargetEffect;
pub use crate::three::TerrainMaterialDesc;
pub use crate::three::TerrainTile;
pub use crate::three::TerrainTileData;
pub use crate::three::Text3D as Text;
pub use crate::three::TextAlign3D as TextAlign;
pub use crate::three::TextureArray;
pub use crate::three::TextureKind;
pub use crate::three::Tonemap;
pub use crate::three::Trail3D as Trail;
pub use crate::three::Transform3D as Transform;
pub use crate::ui::UiPanel3D as UiPanel;
