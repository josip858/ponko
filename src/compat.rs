//! Deprecated root aliases. Canonical module exports use original definitions.
#[doc(hidden)]
#[deprecated(note = "use ponko::access::AccessConflict")]
pub use crate::access::AccessConflict;
#[doc(hidden)]
#[deprecated(note = "use ponko::access::AccessReport")]
pub type AccessReport = crate::access::AccessReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::access::ReportedPair")]
pub type ReportedPair = crate::access::ReportedPair;
#[doc(hidden)]
#[deprecated(note = "use ponko::access::ReportedSystem")]
pub type ReportedSystem = crate::access::ReportedSystem;
#[doc(hidden)]
#[deprecated(note = "use ponko::access::Service")]
pub use crate::access::Service;
#[doc(hidden)]
#[deprecated(note = "use ponko::access::SystemAccess")]
pub type SystemAccess = crate::access::SystemAccess;
#[doc(hidden)]
#[deprecated(note = "use ponko::actions::InputEvent")]
pub use crate::actions::InputEvent;
#[doc(hidden)]
#[deprecated(note = "use ponko::actions::UnknownAction")]
pub type UnknownAction = crate::actions::UnknownAction;
#[doc(hidden)]
#[deprecated(note = "use ponko::system_view::SystemView")]
pub type SystemView<'a> = crate::system_view::SystemView<'a>;
#[doc(hidden)]
#[deprecated(note = "use ponko::system_view::ViewEvents")]
pub type ViewEvents<'a> = crate::system_view::ViewEvents<'a>;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::NavAgent")]
#[cfg(feature = "navmesh")]
pub type NavAgent3D = crate::d3::NavAgent;
#[doc(hidden)]
#[deprecated(note = "use ponko::nav::NavBakeError")]
#[cfg(feature = "navmesh")]
pub type NavBakeError = crate::nav::NavBakeError;
#[doc(hidden)]
#[deprecated(note = "use ponko::nav::NavBakeSettings")]
#[cfg(feature = "navmesh")]
pub type NavBakeSettings = crate::nav::NavBakeSettings;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::NavMesh")]
#[cfg(feature = "navmesh")]
pub type NavMesh3D = crate::d3::NavMesh;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::NavObstacle")]
#[cfg(feature = "navmesh")]
pub type NavObstacle3D = crate::d3::NavObstacle;
#[doc(hidden)]
#[deprecated(note = "use ponko::nav::NavSource")]
#[cfg(feature = "navmesh")]
pub type NavSource = crate::nav::NavSource;
#[doc(hidden)]
#[deprecated(note = "use ponko::alloc_count::AllocCounts")]
pub type AllocCounts = crate::alloc_count::AllocCounts;
#[doc(hidden)]
#[deprecated(note = "use ponko::alloc_count::CountingAllocator")]
pub type CountingAllocator = crate::alloc_count::CountingAllocator;
#[doc(hidden)]
#[deprecated(note = "use ponko::audio::Audio")]
pub type Audio = crate::audio::Audio;
#[doc(hidden)]
#[deprecated(note = "use ponko::audio::AudioEffects")]
pub type AudioEffects = crate::audio::AudioEffects;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::AudioListener")]
#[cfg(feature = "3d")]
pub type AudioListener3D = crate::d3::AudioListener;
#[doc(hidden)]
#[deprecated(note = "use ponko::audio::AudioRolloff")]
#[cfg(feature = "3d")]
pub use crate::audio::AudioRolloff;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::AudioSettings")]
#[cfg(feature = "3d")]
pub type AudioSettings3D = crate::d3::AudioSettings;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::AudioSource")]
#[cfg(feature = "3d")]
pub type AudioSource3D = crate::d3::AudioSource;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Camera")]
pub type Camera2D = crate::d2::Camera;
#[doc(hidden)]
#[deprecated(note = "use ponko::diagnostics::ArchetypeDiagnostics")]
pub type ArchetypeDiagnostics = crate::diagnostics::ArchetypeDiagnostics;
#[doc(hidden)]
#[deprecated(note = "use ponko::diagnostics::ComponentDiagnostics")]
pub type ComponentDiagnostics = crate::diagnostics::ComponentDiagnostics;
#[doc(hidden)]
#[deprecated(note = "use ponko::diagnostics::EcsDiagnostics")]
pub type EcsDiagnostics = crate::diagnostics::EcsDiagnostics;
#[doc(hidden)]
#[deprecated(note = "use ponko::diagnostics::MatchCount")]
pub type MatchCount = crate::diagnostics::MatchCount;
#[doc(hidden)]
#[deprecated(note = "use ponko::diagnostics::SystemDiagnostics")]
pub type SystemDiagnostics = crate::diagnostics::SystemDiagnostics;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Sprite")]
pub type Sprite = crate::d2::Sprite;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Transform")]
pub type Transform2D = crate::d2::Transform;
#[doc(hidden)]
#[deprecated(note = "use ponko::flow::FlowField")]
pub type FlowField = crate::flow::FlowField;
#[doc(hidden)]
#[deprecated(note = "use ponko::flow::NavGrid")]
pub type NavGrid = crate::flow::NavGrid;
#[doc(hidden)]
#[deprecated(note = "use ponko::handle::LiveCounts")]
pub type LiveCounts = crate::handle::LiveCounts;
#[doc(hidden)]
#[deprecated(note = "use ponko::input::Gamepad")]
pub type Gamepad = crate::input::Gamepad;
#[doc(hidden)]
#[deprecated(note = "use ponko::input::GamepadAxis")]
pub use crate::input::GamepadAxis;
#[doc(hidden)]
#[deprecated(note = "use ponko::input::GamepadButton")]
pub use crate::input::GamepadButton;
#[doc(hidden)]
#[deprecated(note = "use ponko::input::GamepadStick")]
pub use crate::input::GamepadStick;
#[doc(hidden)]
#[deprecated(note = "use ponko::input::Input")]
pub type Input = crate::input::Input;
#[doc(hidden)]
#[deprecated(note = "use ponko::inventory::AssetClass")]
pub use crate::inventory::AssetClass;
#[doc(hidden)]
#[deprecated(note = "use ponko::inventory::AssetCount")]
pub type AssetCount = crate::inventory::AssetCount;
#[doc(hidden)]
#[deprecated(note = "use ponko::inventory::AssetGrowth")]
pub type AssetGrowth = crate::inventory::AssetGrowth;
#[doc(hidden)]
#[deprecated(note = "use ponko::inventory::AssetInventory")]
pub type AssetInventory = crate::inventory::AssetInventory;
#[doc(hidden)]
#[deprecated(note = "use ponko::inventory::GpuResidency")]
pub type GpuResidency = crate::inventory::GpuResidency;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::Authority")]
pub type Authority = crate::net::Authority;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::AuthorityCheck")]
pub use crate::net::AuthorityCheck;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::ClockSync")]
pub type ClockSync = crate::net::ClockSync;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::Interest")]
pub type Interest = crate::net::Interest;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::InterestChange")]
pub type InterestChange = crate::net::InterestChange;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::InterestChangeKind")]
pub use crate::net::InterestChangeKind;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::InterestConfig")]
pub type InterestConfig = crate::net::InterestConfig;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::InterestEntity")]
pub type InterestEntity = crate::net::InterestEntity;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::InterestEntry")]
pub type InterestEntry = crate::net::InterestEntry;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::InterestTier")]
pub type InterestTier = crate::net::InterestTier;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::InterestViewer")]
pub type InterestViewer = crate::net::InterestViewer;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::LatestWins")]
pub type LatestWins<K> = crate::net::LatestWins<K>;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::Pose")]
pub type Pose = crate::net::Pose;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SnapshotBuffer")]
pub type SnapshotBuffer<T> = crate::net::SnapshotBuffer<T>;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SnapshotConfig")]
pub type SnapshotConfig = crate::net::SnapshotConfig;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SnapshotPush")]
pub use crate::net::SnapshotPush;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SnapshotSample")]
pub type SnapshotSample<T> = crate::net::SnapshotSample<T>;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SnapshotState")]
pub use crate::net::SnapshotState;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SnapshotStream")]
pub type SnapshotStream<T> = crate::net::SnapshotStream<T>;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::UpdateStamp")]
pub type UpdateStamp = crate::net::UpdateStamp;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::Desync")]
pub type Desync = crate::net::Desync;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::Lane")]
pub use crate::net::Lane;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::MessageEvent")]
pub use crate::net::MessageEvent;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::MessageStats")]
pub type MessageStats = crate::net::MessageStats;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::Net")]
pub type Net = crate::net::Net;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::NetMessages")]
pub type NetMessages = crate::net::NetMessages;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::NetState")]
pub use crate::net::NetState;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::NetStats")]
pub type NetStats = crate::net::NetStats;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::PeerId")]
pub type PeerId = crate::net::PeerId;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SteamLobby")]
#[cfg(feature = "steam")]
pub type SteamLobby = crate::net::SteamLobby;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SteamLobbyVisibility")]
#[cfg(feature = "steam")]
pub use crate::net::SteamLobbyVisibility;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::Perf")]
pub type Perf = crate::perf::Perf;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::SystemTiming")]
pub type SystemTiming = crate::perf::SystemTiming;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::AllocReport")]
pub type AllocReport = crate::perf::AllocReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::Budget")]
pub type Budget = crate::perf::Budget;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::BudgetViolation")]
pub type BudgetViolation = crate::perf::BudgetViolation;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::Contributor")]
pub type Contributor = crate::perf::Contributor;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::FrameTimes")]
pub type FrameTimes = crate::perf::FrameTimes;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::PassReport")]
pub type PassReport = crate::perf::PassReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::PerfReport")]
pub type PerfReport = crate::perf::PerfReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::PerfWarning")]
pub type PerfWarning = crate::perf::PerfWarning;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::Regression")]
pub type Regression = crate::perf::Regression;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::RenderReport")]
pub type RenderReport = crate::perf::RenderReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::ScopeReport")]
pub type ScopeReport = crate::perf::ScopeReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::SystemReport")]
pub type SystemReport = crate::perf::SystemReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::WarningKind")]
pub use crate::perf::WarningKind;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::WorstFrame")]
pub type WorstFrame = crate::perf::WorstFrame;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Collider")]
#[cfg(feature = "physics")]
pub type Collider = crate::d2::Collider;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Physics")]
#[cfg(feature = "physics")]
pub type Physics = crate::d2::Physics;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Hit")]
#[cfg(feature = "physics")]
pub type RayHit = crate::d2::Hit;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::RigidBody")]
#[cfg(feature = "physics")]
pub type RigidBody = crate::d2::RigidBody;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Shape")]
#[cfg(feature = "physics")]
pub use crate::d2::Shape;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Velocity")]
#[cfg(feature = "physics")]
pub type Velocity = crate::d2::Velocity;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Character")]
#[cfg(feature = "physics3d")]
pub type Character3D = crate::d3::Character;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Collider")]
#[cfg(feature = "physics3d")]
pub type Collider3D = crate::d3::Collider;
#[doc(hidden)]
#[deprecated(note = "use ponko::physics3d::ColliderFocus")]
#[cfg(feature = "physics3d")]
pub type ColliderFocus = crate::physics3d::ColliderFocus;
#[doc(hidden)]
#[deprecated(note = "use ponko::physics3d::CollisionGroups")]
#[cfg(feature = "physics3d")]
pub type CollisionGroups = crate::physics3d::CollisionGroups;
#[doc(hidden)]
#[deprecated(note = "use ponko::physics3d::CollisionMesh")]
#[cfg(feature = "physics3d")]
pub type CollisionMesh = crate::physics3d::CollisionMesh;
#[doc(hidden)]
#[deprecated(note = "use ponko::physics3d::CollisionMeshData")]
#[cfg(feature = "physics3d")]
pub type CollisionMeshData = crate::physics3d::CollisionMeshData;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::ContactModifier")]
#[cfg(feature = "physics3d")]
pub type ContactModifier3D = crate::d3::ContactModifier;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::ContactPair")]
#[cfg(feature = "physics3d")]
pub type ContactPair3D<'a, 'b> = crate::d3::ContactPair<'a, 'b>;
#[doc(hidden)]
#[deprecated(note = "use ponko::physics3d::FrictionCurve")]
#[cfg(feature = "physics3d")]
pub type FrictionCurve = crate::physics3d::FrictionCurve;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Hit")]
#[cfg(feature = "physics3d")]
pub type Hit3D = crate::d3::Hit;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Joint")]
#[cfg(feature = "physics3d")]
pub type Joint3D = crate::d3::Joint;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::JointAxis")]
#[cfg(feature = "physics3d")]
pub type JointAxis3D = crate::d3::JointAxis;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::JointDrive")]
#[cfg(feature = "physics3d")]
pub type JointDrive3D = crate::d3::JointDrive;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::JointMotion")]
#[cfg(feature = "physics3d")]
pub type JointMotion3D = crate::d3::JointMotion;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Physics")]
#[cfg(feature = "physics3d")]
pub type Physics3D = crate::d3::Physics;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::QueryFilter")]
#[cfg(feature = "physics3d")]
pub type QueryFilter3D = crate::d3::QueryFilter;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Ragdoll")]
#[cfg(feature = "physics3d")]
pub type Ragdoll3D = crate::d3::Ragdoll;
#[doc(hidden)]
#[deprecated(note = "use ponko::physics3d::RagdollBone")]
#[cfg(feature = "physics3d")]
pub type RagdollBone = crate::physics3d::RagdollBone;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::RigidBody")]
#[cfg(feature = "physics3d")]
pub type RigidBody3D = crate::d3::RigidBody;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Shape")]
#[cfg(feature = "physics3d")]
pub use crate::d3::Shape as Shape3D;
#[doc(hidden)]
#[deprecated(note = "use ponko::physics3d::TerrainCollider")]
#[cfg(feature = "physics3d")]
pub type TerrainCollider = crate::physics3d::TerrainCollider;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Vehicle")]
#[cfg(feature = "physics3d")]
pub type Vehicle3D = crate::d3::Vehicle;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Velocity")]
#[cfg(feature = "physics3d")]
pub type Velocity3D = crate::d3::Velocity;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Wheel")]
#[cfg(feature = "physics3d")]
pub type Wheel3D = crate::d3::Wheel;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::FrameStats")]
pub type FrameStats = crate::perf::FrameStats;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::GpuPass")]
pub type GpuPass = crate::perf::GpuPass;
#[doc(hidden)]
#[deprecated(note = "use ponko::reflect::ReflectError")]
pub use crate::reflect::ReflectError;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuAdapter")]
pub type GpuAdapter = crate::render::GpuAdapter;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuCapabilities")]
pub type GpuCapabilities = crate::render::GpuCapabilities;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuFeature")]
pub use crate::render::GpuFeature;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuFeatureStatus")]
pub type GpuFeatureStatus = crate::render::GpuFeatureStatus;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuLimits")]
pub type GpuLimits = crate::render::GpuLimits;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuTier")]
pub use crate::render::GpuTier;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuUploads")]
pub type GpuUploads = crate::render::GpuUploads;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::PipelineStats")]
pub type PipelineStats = crate::render::PipelineStats;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::RenderStats")]
pub type RenderStats = crate::render::RenderStats;
#[doc(hidden)]
#[deprecated(note = "use ponko::residency::ResidencyKind")]
pub use crate::residency::ResidencyKind;
#[doc(hidden)]
#[deprecated(note = "use ponko::residency::ResidencyStats")]
pub type ResidencyStats = crate::residency::ResidencyStats;
#[doc(hidden)]
#[deprecated(note = "use ponko::scenario::InputScenario")]
pub type InputScenario = crate::scenario::InputScenario;
#[doc(hidden)]
#[deprecated(note = "use ponko::scenario::ScenarioCheck")]
pub use crate::scenario::ScenarioCheck;
#[doc(hidden)]
#[deprecated(note = "use ponko::scenario::ScenarioError")]
pub type ScenarioError = crate::scenario::ScenarioError;
#[doc(hidden)]
#[deprecated(note = "use ponko::scenario::ScenarioFailure")]
pub type ScenarioFailure = crate::scenario::ScenarioFailure;
#[doc(hidden)]
#[deprecated(note = "use ponko::scenario::ScenarioReport")]
pub type ScenarioReport = crate::scenario::ScenarioReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::scenario::ScenarioStatus")]
pub use crate::scenario::ScenarioStatus;
#[doc(hidden)]
#[deprecated(note = "use ponko::scope::ProfileScope")]
pub type ProfileScope = crate::scope::ProfileScope;
#[doc(hidden)]
#[deprecated(note = "use ponko::scope::ScopeGuard")]
pub type ScopeGuard = crate::scope::ScopeGuard;
#[doc(hidden)]
#[deprecated(note = "use ponko::scope::ScopeTiming")]
pub type ScopeTiming = crate::scope::ScopeTiming;
#[doc(hidden)]
#[deprecated(note = "use ponko::spatial::GridEntry")]
pub type GridEntry<T = ()> = crate::spatial::GridEntry<T>;
#[doc(hidden)]
#[deprecated(note = "use ponko::spatial::SpatialGrid")]
pub type SpatialGrid<T = ()> = crate::spatial::SpatialGrid<T>;
#[doc(hidden)]
#[deprecated(note = "use ponko::streaming::AssetState")]
pub use crate::streaming::AssetState;
#[doc(hidden)]
#[deprecated(note = "use ponko::streaming::StreamedAsset")]
pub type StreamedAsset = crate::streaming::StreamedAsset;
#[doc(hidden)]
#[deprecated(note = "use ponko::streaming::UploadStats")]
pub type UploadStats = crate::streaming::UploadStats;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::AmbientOcclusion")]
#[cfg(feature = "3d")]
pub type AmbientOcclusion = crate::render::AmbientOcclusion;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::AnimationCondition")]
#[cfg(feature = "3d")]
pub use crate::anim::AnimationCondition;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::AnimationEvent")]
#[cfg(feature = "3d")]
pub type AnimationEvent = crate::anim::AnimationEvent;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::AnimationGraph")]
#[cfg(feature = "3d")]
pub type AnimationGraph = crate::anim::AnimationGraph;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::AnimationLayer")]
#[cfg(feature = "3d")]
pub type AnimationLayer = crate::anim::AnimationLayer;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::AnimationParameter")]
#[cfg(feature = "3d")]
pub use crate::anim::AnimationParameter;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::AnimationState")]
#[cfg(feature = "3d")]
pub type AnimationState = crate::anim::AnimationState;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::AnimationTrack")]
#[cfg(feature = "3d")]
pub type AnimationTrack = crate::anim::AnimationTrack;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::AnimationTransition")]
#[cfg(feature = "3d")]
pub type AnimationTransition = crate::anim::AnimationTransition;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::Animator")]
#[cfg(feature = "3d")]
pub type Animator = crate::anim::Animator;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::AntiAliasing")]
#[cfg(feature = "3d")]
pub use crate::render::AntiAliasing;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::AttachToJoint")]
#[cfg(feature = "3d")]
pub type AttachToJoint = crate::anim::AttachToJoint;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::Blend")]
#[cfg(feature = "3d")]
pub use crate::anim::Blend;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Camera")]
#[cfg(feature = "3d")]
pub type Camera3D = crate::d3::Camera;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::Compute")]
#[cfg(feature = "3d")]
pub type Compute = crate::render::Compute;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::ComputePass")]
#[cfg(feature = "3d")]
pub type ComputePass = crate::render::ComputePass;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Decal")]
#[cfg(feature = "3d")]
pub type Decal3D = crate::d3::Decal;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::Effect")]
#[cfg(feature = "3d")]
pub type Effect = crate::render::Effect;
#[doc(hidden)]
#[deprecated(note = "use ponko::terrain::FarTerrain")]
#[cfg(feature = "3d")]
pub type FarTerrain = crate::terrain::FarTerrain;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GlobalIllumination")]
#[cfg(feature = "3d")]
pub type GlobalIllumination = crate::render::GlobalIllumination;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuBuffer")]
#[cfg(feature = "3d")]
pub type GpuBuffer = crate::render::GpuBuffer;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GraphicsQuality")]
#[cfg(feature = "3d")]
pub type GraphicsQuality = crate::render::GraphicsQuality;
#[doc(hidden)]
#[deprecated(note = "use ponko::terrain::Heightfield")]
#[cfg(feature = "3d")]
pub type Heightfield = crate::terrain::Heightfield;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::InstanceBatch")]
#[cfg(feature = "3d")]
pub type InstanceBatch = crate::render::InstanceBatch;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::InstanceBatchOptions")]
#[cfg(feature = "3d")]
pub type InstanceBatchOptions = crate::render::InstanceBatchOptions;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::InstanceLevel")]
#[cfg(feature = "3d")]
pub type InstanceLevel = crate::render::InstanceLevel;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::Interpolation")]
#[cfg(feature = "3d")]
pub use crate::anim::Interpolation;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::Joint")]
#[cfg(feature = "3d")]
pub type Joint = crate::anim::Joint;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::JointAnimation")]
#[cfg(feature = "3d")]
pub type JointAnimation = crate::anim::JointAnimation;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::LayerBlend")]
#[cfg(feature = "3d")]
pub use crate::anim::LayerBlend;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::Lighting")]
#[cfg(feature = "3d")]
pub type Lighting = crate::render::Lighting;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Line")]
#[cfg(feature = "3d")]
pub type Line3D = crate::d3::Line;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::LodGroup")]
#[cfg(feature = "3d")]
pub type LodGroup = crate::render::LodGroup;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::LodHysteresis")]
#[cfg(feature = "3d")]
pub use crate::render::LodHysteresis;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::LodLevel")]
#[cfg(feature = "3d")]
pub type LodLevel = crate::render::LodLevel;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::LodPick")]
#[cfg(feature = "3d")]
pub use crate::render::LodPick;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::LodRenderer")]
#[cfg(feature = "3d")]
pub type LodRenderer = crate::render::LodRenderer;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::LodSettings")]
#[cfg(feature = "3d")]
pub type LodSettings = crate::render::LodSettings;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::LodShadow")]
#[cfg(feature = "3d")]
pub use crate::render::LodShadow;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::Material")]
#[cfg(feature = "3d")]
pub use crate::render::Material;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::MaterialDesc")]
#[cfg(feature = "3d")]
pub type MaterialDesc = crate::render::MaterialDesc;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::Mesh")]
#[cfg(feature = "3d")]
pub type Mesh = crate::render::Mesh;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::MeshCost")]
#[cfg(feature = "3d")]
pub type MeshCost = crate::render::MeshCost;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::MeshData")]
#[cfg(feature = "3d")]
pub type MeshData = crate::render::MeshData;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::Model")]
#[cfg(feature = "3d")]
pub type Model = crate::render::Model;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::ModelMaterial")]
#[cfg(feature = "3d")]
pub use crate::render::ModelMaterial;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::MorphAnimation")]
#[cfg(feature = "3d")]
pub type MorphAnimation = crate::anim::MorphAnimation;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::MorphFrame")]
#[cfg(feature = "3d")]
pub type MorphFrame = crate::anim::MorphFrame;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::MorphTarget")]
#[cfg(feature = "3d")]
pub type MorphTarget = crate::anim::MorphTarget;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::Motion")]
#[cfg(feature = "3d")]
pub use crate::anim::Motion;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::MotionChild")]
#[cfg(feature = "3d")]
pub type MotionChild = crate::anim::MotionChild;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::ParticleBlend")]
#[cfg(feature = "3d")]
pub type ParticleBlend3D = crate::d3::ParticleBlend;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::ParticleEmitter")]
#[cfg(feature = "3d")]
pub type ParticleEmitter3D = crate::d3::ParticleEmitter;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::ParticleSettings")]
#[cfg(feature = "3d")]
pub type ParticleSettings3D = crate::d3::ParticleSettings;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::ParticleShape")]
#[cfg(feature = "3d")]
pub type ParticleShape3D = crate::d3::ParticleShape;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::ParticleSpace")]
#[cfg(feature = "3d")]
pub type ParticleSpace3D = crate::d3::ParticleSpace;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::PointLight")]
#[cfg(feature = "3d")]
pub type PointLight3D = crate::d3::PointLight;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::Post")]
#[cfg(feature = "3d")]
pub type Post = crate::render::Post;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::QualityLevel")]
#[cfg(feature = "3d")]
pub use crate::render::QualityLevel;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::ReceivesDecals")]
#[cfg(feature = "3d")]
pub use crate::render::ReceivesDecals;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::ReflectionProbe")]
#[cfg(feature = "3d")]
pub type ReflectionProbe3D = crate::d3::ReflectionProbe;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::RootMotion")]
#[cfg(feature = "3d")]
pub type RootMotion = crate::anim::RootMotion;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::ScreenEffect")]
#[cfg(feature = "3d")]
pub type ScreenEffect = crate::render::ScreenEffect;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::Shader")]
#[cfg(feature = "3d")]
pub type Shader = crate::render::Shader;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::ShaderDiagnostic")]
#[cfg(feature = "3d")]
pub type ShaderDiagnostic = crate::render::ShaderDiagnostic;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::ShaderKind")]
#[cfg(feature = "3d")]
pub use crate::render::ShaderKind;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::ShaderOutcome")]
#[cfg(feature = "3d")]
pub use crate::render::ShaderOutcome;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::ShaderStatus")]
#[cfg(feature = "3d")]
pub type ShaderStatus = crate::render::ShaderStatus;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::Skeleton")]
#[cfg(feature = "3d")]
pub type Skeleton = crate::anim::Skeleton;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::SkinOf")]
#[cfg(feature = "3d")]
pub use crate::anim::SkinOf;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::Sky")]
#[cfg(feature = "3d")]
pub type Sky = crate::render::Sky;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::SpotLight")]
#[cfg(feature = "3d")]
pub type SpotLight3D = crate::d3::SpotLight;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::TargetCamera")]
#[cfg(feature = "3d")]
pub type TargetCamera = crate::render::TargetCamera;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::TargetEffect")]
#[cfg(feature = "3d")]
pub type TargetEffect = crate::render::TargetEffect;
#[doc(hidden)]
#[deprecated(note = "use ponko::terrain::TerrainMaterialDesc")]
#[cfg(feature = "3d")]
pub type TerrainMaterialDesc = crate::terrain::TerrainMaterialDesc;
#[doc(hidden)]
#[deprecated(note = "use ponko::terrain::TerrainTile")]
#[cfg(feature = "3d")]
pub type TerrainTile = crate::terrain::TerrainTile;
#[doc(hidden)]
#[deprecated(note = "use ponko::terrain::TerrainTileData")]
#[cfg(feature = "3d")]
pub type TerrainTileData = crate::terrain::TerrainTileData;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Text")]
#[cfg(feature = "3d")]
pub type Text3D = crate::d3::Text;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::TextAlign")]
#[cfg(feature = "3d")]
pub type TextAlign3D = crate::d3::TextAlign;
#[doc(hidden)]
#[deprecated(note = "use ponko::terrain::TextureArray")]
#[cfg(feature = "3d")]
pub type TextureArray = crate::terrain::TextureArray;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::Tonemap")]
#[cfg(feature = "3d")]
pub use crate::render::Tonemap;
#[doc(hidden)]
#[deprecated(note = "use ponko::terrain::TextureKind")]
#[cfg(feature = "3d")]
pub use crate::terrain::TextureKind;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Trail")]
#[cfg(feature = "3d")]
pub type Trail3D = crate::d3::Trail;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Transform")]
#[cfg(feature = "3d")]
pub type Transform3D = crate::d3::Transform;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::ExtractionBench")]
#[cfg(feature = "3d")]
pub type ExtractionBench = crate::render::ExtractionBench;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::ExtractionSample")]
#[cfg(feature = "3d")]
pub type ExtractionSample = crate::render::ExtractionSample;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::Ui")]
pub type Ui = crate::ui::Ui;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiAccessNode")]
pub type UiAccessNode = crate::ui::UiAccessNode;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiAccessTree")]
pub type UiAccessTree = crate::ui::UiAccessTree;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiAccessValue")]
pub use crate::ui::UiAccessValue;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiBuilder")]
pub type UiBuilder<'a> = crate::ui::UiBuilder<'a>;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiEvent")]
pub use crate::ui::UiEvent;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiId")]
pub type UiId = crate::ui::UiId;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiImmediate")]
pub type UiImmediate = crate::ui::UiImmediate;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiInput")]
pub type UiInput = crate::ui::UiInput;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiLayout")]
pub type UiLayout = crate::ui::UiLayout;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiPaint")]
pub type UiPaint = crate::ui::UiPaint;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiRole")]
pub use crate::ui::UiRole;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiStyle")]
pub type UiStyle = crate::ui::UiStyle;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiTextAlign")]
pub use crate::ui::UiTextAlign;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiTheme")]
pub type UiTheme = crate::ui::UiTheme;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiWidget")]
pub use crate::ui::UiWidget;

#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Decal::MAX")]
#[cfg(feature = "3d")]
pub const MAX_DECALS: usize = crate::d3::Decal::MAX;

#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Decal::MAX_PER_CLUSTER")]
#[cfg(feature = "3d")]
pub const MAX_DECALS_PER_CLUSTER: usize = crate::d3::Decal::MAX_PER_CLUSTER;

#[doc(hidden)]
#[deprecated(note = "use ponko::render::GlobalIllumination::MAX_PROBES")]
#[cfg(feature = "3d")]
pub const MAX_GI_PROBES: u32 = crate::render::GlobalIllumination::MAX_PROBES;

#[doc(hidden)]
#[deprecated(note = "use ponko::render::GlobalIllumination::MAX_PROBES_PER_FRAME")]
#[cfg(feature = "3d")]
pub const MAX_GI_PROBES_PER_FRAME: u32 = crate::render::GlobalIllumination::MAX_PROBES_PER_FRAME;

#[doc(hidden)]
#[deprecated(note = "use ponko::d3::PointLight::MAX_SHADOW_VIEWS")]
#[cfg(feature = "3d")]
pub const MAX_LIGHT_SHADOW_VIEWS: usize = crate::d3::PointLight::MAX_SHADOW_VIEWS;

#[doc(hidden)]
#[deprecated(note = "use ponko::d3::PointLight::SHADOW_SIZE")]
#[cfg(feature = "3d")]
pub const LIGHT_SHADOW_SIZE: u32 = crate::d3::PointLight::SHADOW_SIZE;

#[doc(hidden)]
#[deprecated(note = "use ponko::d3::PointLight::SHADOW_ATLAS_SIZE")]
#[cfg(feature = "3d")]
pub const LIGHT_SHADOW_ATLAS_SIZE: u32 = crate::d3::PointLight::SHADOW_ATLAS_SIZE;

#[doc(hidden)]
#[deprecated(note = "use ponko::render::GraphicsQuality::MIN_RENDER_SCALE")]
#[cfg(feature = "3d")]
pub const MIN_RENDER_SCALE: f32 = crate::render::GraphicsQuality::MIN_RENDER_SCALE;

#[doc(hidden)]
#[deprecated(note = "use ponko::render::GraphicsQuality::MAX_RENDER_SCALE")]
#[cfg(feature = "3d")]
pub const MAX_RENDER_SCALE: f32 = crate::render::GraphicsQuality::MAX_RENDER_SCALE;

#[doc(hidden)]
#[deprecated(note = "use ponko::terrain::TerrainTile::LODS")]
#[cfg(feature = "3d")]
pub const TERRAIN_LODS: usize = crate::terrain::TerrainTile::LODS;

#[doc(hidden)]
#[deprecated(note = "use ponko::terrain::TerrainTile::LAYERS")]
#[cfg(feature = "3d")]
pub const TERRAIN_LAYERS: usize = crate::terrain::TerrainTile::LAYERS;

#[doc(hidden)]
#[deprecated(note = "use ponko::spatial::SpatialGrid::<()>::SORT_EVERY")]
pub const SPATIAL_SORT_EVERY: u64 = crate::spatial::SpatialGrid::<()>::SORT_EVERY;

#[doc(hidden)]
#[deprecated(note = "use ponko::perf::alloc_counts")]
pub fn alloc_counts() -> crate::perf::AllocCounts {
    crate::perf::alloc_counts()
}

#[doc(hidden)]
#[deprecated(note = "use ponko::perf::note_allocation")]
pub fn note_allocation(bytes: usize) {
    crate::perf::note_allocation(bytes)
}

#[doc(hidden)]
#[deprecated(note = "use ponko::camera::camera_controls")]
pub fn camera_controls(ctx: &mut crate::Ctx) {
    crate::camera::camera_controls(ctx)
}

#[doc(hidden)]
#[deprecated(note = "use ponko::net::seq_diff")]
pub const fn seq_diff(a: u16, b: u16) -> i16 {
    crate::net::seq_diff(a, b)
}

#[doc(hidden)]
#[deprecated(note = "use ponko::net::seq_newer")]
pub const fn seq_newer(a: u16, b: u16) -> bool {
    crate::net::seq_newer(a, b)
}

#[doc(hidden)]
#[deprecated(note = "use ponko::perf::scope_named")]
pub fn scope_named(name: &str) -> &'static crate::perf::ProfileScope {
    crate::perf::scope_named(name)
}

#[doc(hidden)]
#[deprecated(note = "use ponko::spatial::spatial_key")]
pub fn spatial_key(position: crate::Vec2, cell_size: f32) -> u64 {
    crate::spatial::spatial_key(position, cell_size)
}

#[doc(hidden)]
#[deprecated(note = "use ponko::anim::two_bone_ik")]
#[cfg(feature = "3d")]
pub fn two_bone_ik(
    s: &mut crate::anim::Skeleton,
    model: &crate::d3::Transform,
    chain: [usize; 3],
    target: crate::Vec3,
    pole: crate::Vec3,
    w: f32,
) {
    crate::anim::two_bone_ik(s, model, chain, target, pole, w)
}

#[doc(hidden)]
#[deprecated(note = "use ponko::anim::look_at_joint")]
#[allow(clippy::too_many_arguments)]
#[cfg(feature = "3d")]
pub fn look_at_joint(
    s: &mut crate::anim::Skeleton,
    model: &crate::d3::Transform,
    joint: usize,
    axis: crate::Vec3,
    target: crate::Vec3,
    max_angle: f32,
    w: f32,
) {
    crate::anim::look_at_joint(s, model, joint, axis, target, max_angle, w)
}

#[doc(hidden)]
#[deprecated(note = "use ponko::net::steam_lobby_from_args")]
#[cfg(feature = "steam")]
pub fn steam_lobby_from_args() -> Option<crate::net::steam::LobbyId> {
    crate::net::steam_lobby_from_args()
}

#[doc(hidden)]
#[deprecated(note = "use ponko::net::steam_wait_for_invite")]
#[cfg(feature = "steam")]
pub fn steam_wait_for_invite(
    client: &crate::net::steam::Client,
    timeout: std::time::Duration,
) -> Option<crate::net::steam::LobbyId> {
    crate::net::steam_wait_for_invite(client, timeout)
}

#[doc(hidden)]
#[deprecated(note = "use ponko::anim::AnimationClip")]
#[cfg(feature = "3d")]
pub type AnimationClip = crate::anim::AnimationClip;

#[doc(hidden)]
#[deprecated(note = "use ponko::anim::JointMask")]
#[cfg(feature = "3d")]
pub type JointMask = crate::anim::JointMask;

#[doc(hidden)]
#[deprecated(note = "use ponko::anim::MorphWeights")]
#[cfg(feature = "3d")]
pub use crate::anim::MorphWeights;

#[doc(hidden)]
#[deprecated(note = "use ponko::render::ModelMaterials")]
#[cfg(feature = "3d")]
pub type ModelMaterials = crate::render::ModelMaterials;

#[doc(hidden)]
#[deprecated(note = "use ponko::render::ParticleCurve")]
#[cfg(feature = "3d")]
pub type ParticleCurve = crate::render::ParticleCurve;

#[doc(hidden)]
#[deprecated(note = "use ponko::render::ParticleGradient")]
#[cfg(feature = "3d")]
pub type ParticleGradient = crate::render::ParticleGradient;

#[doc(hidden)]
#[deprecated(note = "use ponko::render::ShadowMesh")]
#[cfg(feature = "3d")]
pub use crate::render::ShadowMesh;

#[doc(hidden)]
#[deprecated(note = "use ponko::stream::CellStreamer")]
#[cfg(feature = "3d")]
pub type CellStreamer = crate::stream::CellStreamer;

#[doc(hidden)]
#[deprecated(note = "use ponko::stream::Job")]
pub type Job<T> = crate::stream::Job<T>;

#[doc(hidden)]
#[deprecated(note = "use ponko::d3::StreamedLod")]
#[cfg(feature = "3d")]
pub type StreamedLod = crate::d3::StreamedLod;

#[doc(hidden)]
#[deprecated(note = "use ponko::d3::UiPanel")]
#[cfg(feature = "3d")]
pub type UiPanel3D = crate::d3::UiPanel;

#[doc(hidden)]
#[deprecated(note = "use ponko::net::NetPose")]
#[cfg(feature = "3d")]
pub type NetPose = crate::net::NetPose;

macro_rules! moved_consts {
    ($($(#[$cfg:meta])? $name:ident: $ty:ty = $path:path, $note:literal;)*) => {$(
        $(#[$cfg])?
        #[doc(hidden)]
        #[deprecated(note = $note)]
        pub const $name: $ty = $path;
    )*};
}

moved_consts! {
    ECS_DIAGNOSTICS_VERSION: u32 = crate::diagnostics::ECS_DIAGNOSTICS_VERSION, "use ponko::diagnostics::ECS_DIAGNOSTICS_VERSION";
    MAX_GAMEPADS: usize = crate::input::MAX_GAMEPADS, "use ponko::input::MAX_GAMEPADS";
    MAX_INTEREST_TIERS: usize = crate::net::MAX_INTEREST_TIERS, "use ponko::net::MAX_INTEREST_TIERS";
    MAX_INPUT_BYTES: usize = crate::net::MAX_INPUT_BYTES, "use ponko::net::MAX_INPUT_BYTES";
    MAX_MESSAGE_BYTES: usize = crate::net::MAX_MESSAGE_BYTES, "use ponko::net::MAX_MESSAGE_BYTES";
    MAX_PLAYERS: usize = crate::net::MAX_PLAYERS, "use ponko::net::MAX_PLAYERS";
    #[cfg(feature = "physics")]
    MAX_POLYGON_POINTS: usize = crate::physics::MAX_POLYGON_POINTS, "use ponko::physics::MAX_POLYGON_POINTS";
    #[cfg(feature = "physics3d")]
    PHYSICS3D_HZ: u32 = crate::physics3d::PHYSICS3D_HZ, "use ponko::physics3d::PHYSICS3D_HZ";
    DEFAULT_SEED: u64 = crate::random::DEFAULT_SEED, "use ponko::random::DEFAULT_SEED";
    MAX_SCENARIO_STEPS: usize = crate::scenario::MAX_SCENARIO_STEPS, "use ponko::scenario::MAX_SCENARIO_STEPS";
    MAX_SCENARIO_TICKS: u64 = crate::scenario::MAX_SCENARIO_TICKS, "use ponko::scenario::MAX_SCENARIO_TICKS";
    SCENARIO_VERSION: u32 = crate::scenario::SCENARIO_VERSION, "use ponko::scenario::SCENARIO_VERSION";
    DEFAULT_UPLOAD_BUDGET: u64 = crate::streaming::DEFAULT_UPLOAD_BUDGET, "use ponko::streaming::DEFAULT_UPLOAD_BUDGET";
}

#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Attached")]
pub type Attached = crate::attach::Attached;

#[doc(hidden)]
#[deprecated(note = "use ponko::gfx::Graphics")]
pub type Graphics = crate::gfx::Graphics;

#[doc(hidden)]
#[deprecated(note = "use ponko::reflect::Field")]
pub type Field = crate::reflect::Field;

#[doc(hidden)]
#[deprecated(note = "use ponko::d2::TileMap")]
pub type TileMap = crate::tilemap::TileMap;

#[doc(hidden)]
#[deprecated(note = "use ponko::ecs_core::ComponentId")]
pub type ComponentId = ponko_ecs::ComponentId;

#[doc(hidden)]
#[deprecated(note = "use ponko::ecs_core::WorldView")]
pub type WorldView<'w> = ponko_ecs::WorldView<'w>;

#[doc(hidden)]
#[deprecated(note = "use ponko::time::Instant")]
pub type Instant = web_time::Instant;
