//! The go-to-point-on-route task: a ped walks towards a point on a route,
//! and the task advances its progress and speed as it goes.
//!
//! The family is the simple move task of the go-to-point class with a route
//! variant. Its verified 32-bit rewrites are the proof's reference; this
//! module restates the verified methods as methods of
//! [`task::GoToPointOnRouteTask`]. Every call a verified method makes that is
//! not itself lifted is one method of [`host::GoToPointOnRouteHost`]: the
//! host performs the call and gives back its answer, and the lifted method
//! makes the same calls, with the same arguments, in the same order.
//!
//! What is lifted and proven, what is not, and what each proof narrows is
//! listed in [`registry`].
//!
//! No `unsafe` code, no addresses, no numbered virtual slots and no global
//! state: the lifted type owns its data and the host supplies the rest.

#![forbid(unsafe_code)]

pub mod host;
pub mod registry;
pub mod task;

pub use host::{CopyArgs, GoToPointOnRouteHost, TaskField};
pub use task::GoToPointOnRouteTask;
