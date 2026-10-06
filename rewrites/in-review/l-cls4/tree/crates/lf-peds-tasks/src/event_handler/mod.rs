//! Lifted pedestrian event handler.
//!
//! The original decides how a pedestrian responds to the events it
//! receives through one handler class: each virtual slot takes an event,
//! and either clears the handler's pending task, forwards the event to a
//! dispatch slot, or converts the event into a new task through a shared
//! factory. This module owns the handler's data as ordinary Rust (no
//! addresses, no virtual tables, no global state) and restates each
//! verified 32-bit method with behaviour in it as a method on
//! [`EventHandler`].
//!
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results and every
//! effect (see the `lf-evthandler-diff` test crate). Nothing here is
//! verified by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].

#![forbid(unsafe_code)]

mod handler;

pub mod registry;

pub use handler::{
    FIXED_REQUEST_A, FIXED_REQUEST_B, KIND_CLEAR, KIND_RESET_B, KIND_TYPE_CLEAR_B, ConvertRequest,
    EventDispatch, EventHandler, EventPayload, EventSource, FactoryHandle, FactoryState, Owner,
    Task, TaskFactory, TaskManager,
};
