//! Deprecated root aliases. Canonical module exports use original definitions.
#[doc(hidden)]
#[doc(hidden)]
#[deprecated(note = "use ponko::access::AccessConflict")]
#[cfg(feature = "3d")]
pub type AccessConflict = crate::access::AccessConflict;
#[doc(hidden)]
#[deprecated(note = "use ponko::access::AccessReport")]
#[cfg(feature = "3d")]
pub type AccessReport = crate::access::AccessReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::access::ReportedPair")]
#[cfg(feature = "3d")]
pub type ReportedPair = crate::access::ReportedPair;
#[doc(hidden)]
#[deprecated(note = "use ponko::access::ReportedSystem")]
#[cfg(feature = "3d")]
pub type ReportedSystem = crate::access::ReportedSystem;
#[doc(hidden)]
#[deprecated(note = "use ponko::access::Service")]
#[cfg(feature = "3d")]
pub type Service = crate::access::Service;
#[doc(hidden)]
#[deprecated(note = "use ponko::access::SystemAccess")]
#[cfg(feature = "3d")]
pub type SystemAccess = crate::access::SystemAccess;
#[doc(hidden)]
#[deprecated(note = "use ponko::actions::InputEvent")]
#[cfg(feature = "3d")]
pub type InputEvent = crate::actions::InputEvent;
#[doc(hidden)]
#[deprecated(note = "use ponko::actions::UnknownAction")]
#[cfg(feature = "3d")]
pub type UnknownAction = crate::actions::UnknownAction;
#[doc(hidden)]
#[doc(hidden)]
#[deprecated(note = "use ponko::system_view::SystemView")]
#[cfg(feature = "3d")]
pub type SystemView<'a> = crate::system_view::SystemView<'a>;
#[doc(hidden)]
#[deprecated(note = "use ponko::system_view::ViewEvents")]
#[cfg(feature = "3d")]
pub type ViewEvents<'a> = crate::system_view::ViewEvents<'a>;
#[doc(hidden)]
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::NavAgent")]
#[cfg(feature = "navmesh")]
#[cfg(feature = "3d")]
pub type NavAgent3D = crate::d3::NavAgent;
#[doc(hidden)]
#[deprecated(note = "use ponko::nav::NavBakeError")]
#[cfg(feature = "navmesh")]
#[cfg(feature = "3d")]
pub type NavBakeError = crate::nav::NavBakeError;
#[doc(hidden)]
#[deprecated(note = "use ponko::nav::NavBakeSettings")]
#[cfg(feature = "navmesh")]
#[cfg(feature = "3d")]
pub type NavBakeSettings = crate::nav::NavBakeSettings;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::NavMesh")]
#[cfg(feature = "navmesh")]
#[cfg(feature = "3d")]
pub type NavMesh3D = crate::d3::NavMesh;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::NavObstacle")]
#[cfg(feature = "navmesh")]
#[cfg(feature = "3d")]
pub type NavObstacle3D = crate::d3::NavObstacle;
#[doc(hidden)]
#[deprecated(note = "use ponko::nav::NavSource")]
#[cfg(feature = "navmesh")]
#[cfg(feature = "3d")]
pub type NavSource = crate::nav::NavSource;
#[doc(hidden)]
#[deprecated(note = "use ponko::alloc_count::AllocCounts")]
#[cfg(feature = "3d")]
pub type AllocCounts = crate::alloc_count::AllocCounts;
#[doc(hidden)]
#[deprecated(note = "use ponko::alloc_count::CountingAllocator")]
#[cfg(feature = "3d")]
pub type CountingAllocator = crate::alloc_count::CountingAllocator;
#[doc(hidden)]
#[deprecated(note = "use ponko::audio::Audio")]
#[cfg(feature = "3d")]
pub type Audio = crate::audio::Audio;
#[doc(hidden)]
#[deprecated(note = "use ponko::audio::AudioEffects")]
#[cfg(feature = "3d")]
pub type AudioEffects = crate::audio::AudioEffects;
#[doc(hidden)]
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::AudioListener")]
#[cfg(feature = "3d")]
pub type AudioListener3D = crate::d3::AudioListener;
#[doc(hidden)]
#[deprecated(note = "use ponko::audio::AudioRolloff")]
#[cfg(feature = "3d")]
pub type AudioRolloff = crate::audio::AudioRolloff;
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
#[cfg(feature = "3d")]
pub type Camera2D = crate::d2::Camera;
#[doc(hidden)]
#[deprecated(note = "use ponko::diagnostics::ArchetypeDiagnostics")]
#[cfg(feature = "3d")]
pub type ArchetypeDiagnostics = crate::diagnostics::ArchetypeDiagnostics;
#[doc(hidden)]
#[deprecated(note = "use ponko::diagnostics::ComponentDiagnostics")]
#[cfg(feature = "3d")]
pub type ComponentDiagnostics = crate::diagnostics::ComponentDiagnostics;
#[doc(hidden)]
#[deprecated(note = "use ponko::diagnostics::EcsDiagnostics")]
#[cfg(feature = "3d")]
pub type EcsDiagnostics = crate::diagnostics::EcsDiagnostics;
#[doc(hidden)]
#[deprecated(note = "use ponko::diagnostics::MatchCount")]
#[cfg(feature = "3d")]
pub type MatchCount = crate::diagnostics::MatchCount;
#[doc(hidden)]
#[deprecated(note = "use ponko::diagnostics::SystemDiagnostics")]
#[cfg(feature = "3d")]
pub type SystemDiagnostics = crate::diagnostics::SystemDiagnostics;
#[doc(hidden)]
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Sprite")]
#[cfg(feature = "3d")]
pub type Sprite = crate::d2::Sprite;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Transform")]
#[cfg(feature = "3d")]
pub type Transform2D = crate::d2::Transform;
#[doc(hidden)]
#[doc(hidden)]
#[deprecated(note = "use ponko::flow::FlowField")]
#[cfg(feature = "3d")]
pub type FlowField = crate::flow::FlowField;
#[doc(hidden)]
#[deprecated(note = "use ponko::flow::NavGrid")]
#[cfg(feature = "3d")]
pub type NavGrid = crate::flow::NavGrid;
#[doc(hidden)]
#[deprecated(note = "use ponko::handle::LiveCounts")]
#[cfg(feature = "3d")]
pub type LiveCounts = crate::handle::LiveCounts;
#[doc(hidden)]
#[deprecated(note = "use ponko::input::Gamepad")]
#[cfg(feature = "3d")]
pub type Gamepad = crate::input::Gamepad;
#[doc(hidden)]
#[deprecated(note = "use ponko::input::GamepadAxis")]
#[cfg(feature = "3d")]
pub type GamepadAxis = crate::input::GamepadAxis;
#[doc(hidden)]
#[deprecated(note = "use ponko::input::GamepadButton")]
#[cfg(feature = "3d")]
pub type GamepadButton = crate::input::GamepadButton;
#[doc(hidden)]
#[deprecated(note = "use ponko::input::GamepadStick")]
#[cfg(feature = "3d")]
pub type GamepadStick = crate::input::GamepadStick;
#[doc(hidden)]
#[deprecated(note = "use ponko::input::Input")]
#[cfg(feature = "3d")]
pub type Input = crate::input::Input;
#[doc(hidden)]
#[doc(hidden)]
#[deprecated(note = "use ponko::inventory::AssetClass")]
#[cfg(feature = "3d")]
pub type AssetClass = crate::inventory::AssetClass;
#[doc(hidden)]
#[deprecated(note = "use ponko::inventory::AssetCount")]
#[cfg(feature = "3d")]
pub type AssetCount = crate::inventory::AssetCount;
#[doc(hidden)]
#[deprecated(note = "use ponko::inventory::AssetGrowth")]
#[cfg(feature = "3d")]
pub type AssetGrowth = crate::inventory::AssetGrowth;
#[doc(hidden)]
#[deprecated(note = "use ponko::inventory::AssetInventory")]
#[cfg(feature = "3d")]
pub type AssetInventory = crate::inventory::AssetInventory;
#[doc(hidden)]
#[deprecated(note = "use ponko::inventory::GpuResidency")]
#[cfg(feature = "3d")]
pub type GpuResidency = crate::inventory::GpuResidency;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::Authority")]
#[cfg(feature = "3d")]
pub type Authority = crate::net::Authority;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::AuthorityCheck")]
#[cfg(feature = "3d")]
pub type AuthorityCheck = crate::net::AuthorityCheck;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::ClockSync")]
#[cfg(feature = "3d")]
pub type ClockSync = crate::net::ClockSync;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::Interest")]
#[cfg(feature = "3d")]
pub type Interest = crate::net::Interest;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::InterestChange")]
#[cfg(feature = "3d")]
pub type InterestChange = crate::net::InterestChange;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::InterestChangeKind")]
#[cfg(feature = "3d")]
pub type InterestChangeKind = crate::net::InterestChangeKind;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::InterestConfig")]
#[cfg(feature = "3d")]
pub type InterestConfig = crate::net::InterestConfig;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::InterestEntity")]
#[cfg(feature = "3d")]
pub type InterestEntity = crate::net::InterestEntity;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::InterestEntry")]
#[cfg(feature = "3d")]
pub type InterestEntry = crate::net::InterestEntry;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::InterestTier")]
#[cfg(feature = "3d")]
pub type InterestTier = crate::net::InterestTier;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::InterestViewer")]
#[cfg(feature = "3d")]
pub type InterestViewer = crate::net::InterestViewer;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::LatestWins")]
#[cfg(feature = "3d")]
pub type LatestWins<K> = crate::net::LatestWins<K>;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::Pose")]
#[cfg(feature = "3d")]
pub type Pose = crate::net::Pose;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SnapshotBuffer")]
#[cfg(feature = "3d")]
pub type SnapshotBuffer<T> = crate::net::SnapshotBuffer<T>;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SnapshotConfig")]
#[cfg(feature = "3d")]
pub type SnapshotConfig = crate::net::SnapshotConfig;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SnapshotPush")]
#[cfg(feature = "3d")]
pub type SnapshotPush = crate::net::SnapshotPush;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SnapshotSample")]
#[cfg(feature = "3d")]
pub type SnapshotSample<T> = crate::net::SnapshotSample<T>;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SnapshotState")]
#[cfg(feature = "3d")]
pub type SnapshotState = crate::net::SnapshotState;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SnapshotStream")]
#[cfg(feature = "3d")]
pub type SnapshotStream<T> = crate::net::SnapshotStream<T>;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::UpdateStamp")]
#[cfg(feature = "3d")]
pub type UpdateStamp = crate::net::UpdateStamp;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::Desync")]
#[cfg(feature = "3d")]
pub type Desync = crate::net::Desync;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::Lane")]
#[cfg(feature = "3d")]
pub type Lane = crate::net::Lane;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::MessageEvent")]
#[cfg(feature = "3d")]
pub type MessageEvent = crate::net::MessageEvent;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::MessageStats")]
#[cfg(feature = "3d")]
pub type MessageStats = crate::net::MessageStats;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::Net")]
#[cfg(feature = "3d")]
pub type Net = crate::net::Net;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::NetMessages")]
#[cfg(feature = "3d")]
pub type NetMessages = crate::net::NetMessages;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::NetState")]
#[cfg(feature = "3d")]
pub type NetState = crate::net::NetState;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::NetStats")]
#[cfg(feature = "3d")]
pub type NetStats = crate::net::NetStats;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::PeerId")]
#[cfg(feature = "3d")]
pub type PeerId = crate::net::PeerId;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SteamLobby")]
#[cfg(feature = "steam")]
#[cfg(feature = "3d")]
pub type SteamLobby = crate::net::SteamLobby;
#[doc(hidden)]
#[deprecated(note = "use ponko::net::SteamLobbyVisibility")]
#[cfg(feature = "steam")]
#[cfg(feature = "3d")]
pub type SteamLobbyVisibility = crate::net::SteamLobbyVisibility;
#[doc(hidden)]
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::Perf")]
#[cfg(feature = "3d")]
pub type Perf = crate::perf::Perf;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::SystemTiming")]
#[cfg(feature = "3d")]
pub type SystemTiming = crate::perf::SystemTiming;
#[doc(hidden)]
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::AllocReport")]
#[cfg(feature = "3d")]
pub type AllocReport = crate::perf::AllocReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::Budget")]
#[cfg(feature = "3d")]
pub type Budget = crate::perf::Budget;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::BudgetViolation")]
#[cfg(feature = "3d")]
pub type BudgetViolation = crate::perf::BudgetViolation;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::Contributor")]
#[cfg(feature = "3d")]
pub type Contributor = crate::perf::Contributor;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::FrameTimes")]
#[cfg(feature = "3d")]
pub type FrameTimes = crate::perf::FrameTimes;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::PassReport")]
#[cfg(feature = "3d")]
pub type PassReport = crate::perf::PassReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::PerfReport")]
#[cfg(feature = "3d")]
pub type PerfReport = crate::perf::PerfReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::PerfWarning")]
#[cfg(feature = "3d")]
pub type PerfWarning = crate::perf::PerfWarning;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::Regression")]
#[cfg(feature = "3d")]
pub type Regression = crate::perf::Regression;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::RenderReport")]
#[cfg(feature = "3d")]
pub type RenderReport = crate::perf::RenderReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::ScopeReport")]
#[cfg(feature = "3d")]
pub type ScopeReport = crate::perf::ScopeReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::SystemReport")]
#[cfg(feature = "3d")]
pub type SystemReport = crate::perf::SystemReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::WarningKind")]
#[cfg(feature = "3d")]
pub type WarningKind = crate::perf::WarningKind;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::WorstFrame")]
#[cfg(feature = "3d")]
pub type WorstFrame = crate::perf::WorstFrame;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Collider")]
#[cfg(feature = "physics")]
#[cfg(feature = "3d")]
pub type Collider = crate::d2::Collider;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Physics")]
#[cfg(feature = "physics")]
#[cfg(feature = "3d")]
pub type Physics = crate::d2::Physics;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Hit")]
#[cfg(feature = "physics")]
#[cfg(feature = "3d")]
pub type RayHit = crate::d2::Hit;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::RigidBody")]
#[cfg(feature = "physics")]
#[cfg(feature = "3d")]
pub type RigidBody = crate::d2::RigidBody;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Shape")]
#[cfg(feature = "physics")]
#[cfg(feature = "3d")]
pub type Shape = crate::d2::Shape;
#[doc(hidden)]
#[deprecated(note = "use ponko::d2::Velocity")]
#[cfg(feature = "physics")]
#[cfg(feature = "3d")]
pub type Velocity = crate::d2::Velocity;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Character")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type Character3D = crate::d3::Character;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Collider")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type Collider3D = crate::d3::Collider;
#[doc(hidden)]
#[deprecated(note = "use ponko::physics3d::ColliderFocus")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type ColliderFocus = crate::physics3d::ColliderFocus;
#[doc(hidden)]
#[deprecated(note = "use ponko::physics3d::CollisionGroups")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type CollisionGroups = crate::physics3d::CollisionGroups;
#[doc(hidden)]
#[deprecated(note = "use ponko::physics3d::CollisionMesh")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type CollisionMesh = crate::physics3d::CollisionMesh;
#[doc(hidden)]
#[deprecated(note = "use ponko::physics3d::CollisionMeshData")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type CollisionMeshData = crate::physics3d::CollisionMeshData;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::ContactModifier")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type ContactModifier3D = crate::d3::ContactModifier;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::ContactPair")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type ContactPair3D<'a, 'b> = crate::d3::ContactPair<'a, 'b>;
#[doc(hidden)]
#[deprecated(note = "use ponko::physics3d::FrictionCurve")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type FrictionCurve = crate::physics3d::FrictionCurve;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Hit")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type Hit3D = crate::d3::Hit;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Joint")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type Joint3D = crate::d3::Joint;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::JointAxis")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type JointAxis3D = crate::d3::JointAxis;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::JointDrive")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type JointDrive3D = crate::d3::JointDrive;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::JointMotion")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type JointMotion3D = crate::d3::JointMotion;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Physics")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type Physics3D = crate::d3::Physics;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::QueryFilter")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type QueryFilter3D = crate::d3::QueryFilter;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Ragdoll")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type Ragdoll3D = crate::d3::Ragdoll;
#[doc(hidden)]
#[deprecated(note = "use ponko::physics3d::RagdollBone")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type RagdollBone = crate::physics3d::RagdollBone;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::RigidBody")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type RigidBody3D = crate::d3::RigidBody;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Shape")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type Shape3D = crate::d3::Shape;
#[doc(hidden)]
#[deprecated(note = "use ponko::physics3d::TerrainCollider")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type TerrainCollider = crate::physics3d::TerrainCollider;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Vehicle")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type Vehicle3D = crate::d3::Vehicle;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Velocity")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type Velocity3D = crate::d3::Velocity;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Wheel")]
#[cfg(feature = "physics3d")]
#[cfg(feature = "3d")]
pub type Wheel3D = crate::d3::Wheel;
#[doc(hidden)]
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::FrameStats")]
#[cfg(feature = "3d")]
pub type FrameStats = crate::perf::FrameStats;
#[doc(hidden)]
#[deprecated(note = "use ponko::perf::GpuPass")]
#[cfg(feature = "3d")]
pub type GpuPass = crate::perf::GpuPass;
#[doc(hidden)]
#[deprecated(note = "use ponko::reflect::ReflectError")]
#[cfg(feature = "3d")]
pub type ReflectError = crate::reflect::ReflectError;
#[doc(hidden)]
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuAdapter")]
#[cfg(feature = "3d")]
pub type GpuAdapter = crate::render::GpuAdapter;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuCapabilities")]
#[cfg(feature = "3d")]
pub type GpuCapabilities = crate::render::GpuCapabilities;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuFeature")]
#[cfg(feature = "3d")]
pub type GpuFeature = crate::render::GpuFeature;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuFeatureStatus")]
#[cfg(feature = "3d")]
pub type GpuFeatureStatus = crate::render::GpuFeatureStatus;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuLimits")]
#[cfg(feature = "3d")]
pub type GpuLimits = crate::render::GpuLimits;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuTier")]
#[cfg(feature = "3d")]
pub type GpuTier = crate::render::GpuTier;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::GpuUploads")]
#[cfg(feature = "3d")]
pub type GpuUploads = crate::render::GpuUploads;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::PipelineStats")]
#[cfg(feature = "3d")]
pub type PipelineStats = crate::render::PipelineStats;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::RenderStats")]
#[cfg(feature = "3d")]
pub type RenderStats = crate::render::RenderStats;
#[doc(hidden)]
#[doc(hidden)]
#[deprecated(note = "use ponko::residency::ResidencyKind")]
#[cfg(feature = "3d")]
pub type ResidencyKind = crate::residency::ResidencyKind;
#[doc(hidden)]
#[deprecated(note = "use ponko::residency::ResidencyStats")]
#[cfg(feature = "3d")]
pub type ResidencyStats = crate::residency::ResidencyStats;
#[doc(hidden)]
#[deprecated(note = "use ponko::scenario::InputScenario")]
#[cfg(feature = "3d")]
pub type InputScenario = crate::scenario::InputScenario;
#[doc(hidden)]
#[deprecated(note = "use ponko::scenario::ScenarioCheck")]
#[cfg(feature = "3d")]
pub type ScenarioCheck = crate::scenario::ScenarioCheck;
#[doc(hidden)]
#[deprecated(note = "use ponko::scenario::ScenarioError")]
#[cfg(feature = "3d")]
pub type ScenarioError = crate::scenario::ScenarioError;
#[doc(hidden)]
#[deprecated(note = "use ponko::scenario::ScenarioFailure")]
#[cfg(feature = "3d")]
pub type ScenarioFailure = crate::scenario::ScenarioFailure;
#[doc(hidden)]
#[deprecated(note = "use ponko::scenario::ScenarioReport")]
#[cfg(feature = "3d")]
pub type ScenarioReport = crate::scenario::ScenarioReport;
#[doc(hidden)]
#[deprecated(note = "use ponko::scenario::ScenarioStatus")]
#[cfg(feature = "3d")]
pub type ScenarioStatus = crate::scenario::ScenarioStatus;
#[doc(hidden)]
#[deprecated(note = "use ponko::scope::ProfileScope")]
#[cfg(feature = "3d")]
pub type ProfileScope = crate::scope::ProfileScope;
#[doc(hidden)]
#[deprecated(note = "use ponko::scope::ScopeGuard")]
#[cfg(feature = "3d")]
pub type ScopeGuard = crate::scope::ScopeGuard;
#[doc(hidden)]
#[deprecated(note = "use ponko::scope::ScopeTiming")]
#[cfg(feature = "3d")]
pub type ScopeTiming = crate::scope::ScopeTiming;
#[doc(hidden)]
#[deprecated(note = "use ponko::spatial::GridEntry")]
#[cfg(feature = "3d")]
pub type GridEntry<T = ()> = crate::spatial::GridEntry<T>;
#[doc(hidden)]
#[deprecated(note = "use ponko::spatial::SpatialGrid")]
#[cfg(feature = "3d")]
pub type SpatialGrid<T = ()> = crate::spatial::SpatialGrid<T>;
#[doc(hidden)]
#[deprecated(note = "use ponko::streaming::AssetState")]
#[cfg(feature = "3d")]
pub type AssetState = crate::streaming::AssetState;
#[doc(hidden)]
#[deprecated(note = "use ponko::streaming::StreamedAsset")]
#[cfg(feature = "3d")]
pub type StreamedAsset = crate::streaming::StreamedAsset;
#[doc(hidden)]
#[deprecated(note = "use ponko::streaming::UploadStats")]
#[cfg(feature = "3d")]
pub type UploadStats = crate::streaming::UploadStats;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::AmbientOcclusion")]
#[cfg(feature = "3d")]
pub type AmbientOcclusion = crate::render::AmbientOcclusion;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::AnimationCondition")]
#[cfg(feature = "3d")]
pub type AnimationCondition = crate::anim::AnimationCondition;
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
pub type AnimationParameter = crate::anim::AnimationParameter;
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
pub type AntiAliasing = crate::render::AntiAliasing;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::AttachToJoint")]
#[cfg(feature = "3d")]
pub type AttachToJoint = crate::anim::AttachToJoint;
#[doc(hidden)]
#[deprecated(note = "use ponko::anim::Blend")]
#[cfg(feature = "3d")]
pub type Blend = crate::anim::Blend;
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
pub type Interpolation = crate::anim::Interpolation;
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
pub type LayerBlend = crate::anim::LayerBlend;
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
pub type LodHysteresis = crate::render::LodHysteresis;
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use ponko::render::LodHysteresis")]
#[cfg(feature = "3d")]
pub fn LodHysteresis(v0: f32) -> crate::render::LodHysteresis {
    crate::render::LodHysteresis(v0)
}
#[doc(hidden)]
#[deprecated(note = "use ponko::render::LodLevel")]
#[cfg(feature = "3d")]
pub type LodLevel = crate::render::LodLevel;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::LodPick")]
#[cfg(feature = "3d")]
pub type LodPick = crate::render::LodPick;
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
pub type LodShadow = crate::render::LodShadow;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::Material")]
#[cfg(feature = "3d")]
pub type Material = crate::render::Material;
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
pub type ModelMaterial = crate::render::ModelMaterial;
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use ponko::render::ModelMaterial")]
#[cfg(feature = "3d")]
pub fn Material(v0: crate::render::Material) -> crate::render::ModelMaterial {
    crate::render::ModelMaterial(v0)
}
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
pub type Motion = crate::anim::Motion;
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
pub type QualityLevel = crate::render::QualityLevel;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::ReceivesDecals")]
#[cfg(feature = "3d")]
pub type ReceivesDecals = crate::render::ReceivesDecals;
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use ponko::render::ReceivesDecals")]
#[cfg(feature = "3d")]
pub fn ReceivesDecals(v0: u32) -> crate::render::ReceivesDecals {
    crate::render::ReceivesDecals(v0)
}
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
pub type ShaderKind = crate::render::ShaderKind;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::ShaderOutcome")]
#[cfg(feature = "3d")]
pub type ShaderOutcome = crate::render::ShaderOutcome;
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
pub type SkinOf = crate::anim::SkinOf;
#[doc(hidden)]
#[allow(non_snake_case)]
#[deprecated(note = "use ponko::anim::SkinOf")]
#[cfg(feature = "3d")]
pub fn SkinOf(v0: crate::ecs_core::Entity) -> crate::anim::SkinOf {
    crate::anim::SkinOf(v0)
}
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
#[deprecated(note = "use ponko::terrain::TextureKind")]
#[cfg(feature = "3d")]
pub type TextureKind = crate::terrain::TextureKind;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::Tonemap")]
#[cfg(feature = "3d")]
pub type Tonemap = crate::render::Tonemap;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Trail")]
#[cfg(feature = "3d")]
pub type Trail3D = crate::d3::Trail;
#[doc(hidden)]
#[deprecated(note = "use ponko::d3::Transform")]
#[cfg(feature = "3d")]
pub type Transform3D = crate::d3::Transform;
#[doc(hidden)]
#[doc(hidden)]
#[doc(hidden)]
#[deprecated(note = "use ponko::render::ExtractionBench")]
#[cfg(feature = "3d")]
pub type ExtractionBench = crate::render::ExtractionBench;
#[doc(hidden)]
#[deprecated(note = "use ponko::render::ExtractionSample")]
#[cfg(feature = "3d")]
pub type ExtractionSample = crate::render::ExtractionSample;
#[doc(hidden)]
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::Ui")]
#[cfg(feature = "3d")]
pub type Ui = crate::ui::Ui;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiAccessNode")]
#[cfg(feature = "3d")]
pub type UiAccessNode = crate::ui::UiAccessNode;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiAccessTree")]
#[cfg(feature = "3d")]
pub type UiAccessTree = crate::ui::UiAccessTree;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiAccessValue")]
#[cfg(feature = "3d")]
pub type UiAccessValue = crate::ui::UiAccessValue;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiBuilder")]
#[cfg(feature = "3d")]
pub type UiBuilder<'a> = crate::ui::UiBuilder<'a>;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiEvent")]
#[cfg(feature = "3d")]
pub type UiEvent = crate::ui::UiEvent;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiId")]
#[cfg(feature = "3d")]
pub type UiId = crate::ui::UiId;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiImmediate")]
#[cfg(feature = "3d")]
pub type UiImmediate = crate::ui::UiImmediate;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiInput")]
#[cfg(feature = "3d")]
pub type UiInput = crate::ui::UiInput;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiLayout")]
#[cfg(feature = "3d")]
pub type UiLayout = crate::ui::UiLayout;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiPaint")]
#[cfg(feature = "3d")]
pub type UiPaint = crate::ui::UiPaint;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiRole")]
#[cfg(feature = "3d")]
pub type UiRole = crate::ui::UiRole;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiStyle")]
#[cfg(feature = "3d")]
pub type UiStyle = crate::ui::UiStyle;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiTextAlign")]
#[cfg(feature = "3d")]
pub type UiTextAlign = crate::ui::UiTextAlign;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiTheme")]
#[cfg(feature = "3d")]
pub type UiTheme = crate::ui::UiTheme;
#[doc(hidden)]
#[deprecated(note = "use ponko::ui::UiWidget")]
#[cfg(feature = "3d")]
pub type UiWidget = crate::ui::UiWidget;
