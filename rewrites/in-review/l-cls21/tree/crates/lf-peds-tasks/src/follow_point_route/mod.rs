//! The follow-point route task: a ped walks a route of points, one leg at a
//! time, and the task system is asked for the order that matches each step.
//!
//! The family is the complex move task that follows a point route. Its
//! verified 32-bit rewrites are the proof's reference; this module restates
//! the verified methods as methods of [`task::FollowPointRouteTask`], with
//! every call they make left to the [`host::RouteTaskHost`] trait.
//!
//! What is lifted and proven, what is not, and what each proof narrows is
//! listed in [`registry`].
//!
//! No `unsafe` code, no addresses, no numbered virtual slots and no global
//! state: the lifted types own their data and the host supplies the rest.

pub mod host;
pub mod registry;
pub mod route;
pub mod task;

pub use host::{PedView, RoutePollHost, RouteTaskHost};
pub use route::{MAX_POINTS, RoutePoint, RouteTable};
pub use task::FollowPointRouteTask;
