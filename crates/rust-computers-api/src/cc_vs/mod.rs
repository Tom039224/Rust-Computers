//! CC-VS (Valkyrien Skies) ペリフェラル群。
//! CC-VS (Valkyrien Skies) peripheral modules.

pub mod aerodynamics;
pub mod drag;
pub mod ship;

pub use aerodynamics::{Aerodynamics, VSAtmosphericParameters};
pub use drag::Drag;
pub use ship::{
    Ship, VSInertiaInfo, VSJoint, VSPhysicsTickData, VSPoseVelInfo, VSQuaternion,
    VSTeleportData, VSTransformMatrix, VSVector3,
};
