//! The move-between-points scenario family, lifted from its verified rewrites.
//!
//! The scenario task walks a pedestrian between scenario points. This module
//! holds the scenario's own data ([`MoveBetweenPointsScenario`]), the
//! collaborators its methods call ([`MoveBetweenPointsHost`], [`SubTask`]), and
//! the registry of which methods are lifted and what each proof narrows.
//!
//! Only verified methods are lifted. The methods the registry lists as missing
//! are not written here.

mod host;
mod scenario;
mod tick;

pub mod registry;

pub use host::{MoveBetweenPointsHost, SubTask};
pub use scenario::{
    ACTIVE_WEIGHT, IDLE_WEIGHT, MoveBetweenPointsScenario, STATUS_DONE, STATUS_STICKY,
};
pub use tick::{AUX_CHAT, AUX_NARROW, RouteFirst, RouteSecond, TickHost, WorldQuery};
