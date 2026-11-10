//! Navigation in two dimensions and on a 3D navigation mesh.
pub use crate::flow::{FlowField, NavGrid};
#[cfg(feature = "navmesh")]
pub use crate::nav3d::{
    NavAgent3D as Agent, NavBakeError, NavBakeSettings, NavMesh3D as Mesh, NavObstacle3D as Obstacle, NavSource,
    step_navmesh3d,
};
