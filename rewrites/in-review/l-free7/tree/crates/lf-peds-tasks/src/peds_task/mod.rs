//! Lifted free functions of the ped task system: weighted picking, reaction codes, shared state.
//!
//! The naming lanes grouped a further set of task routines by behaviour
//! under `peds_task_*`. None belongs to a named class, and (as the triage
//! in the lane report shows) neither do they belong to one structure:
//! this module owns the data of the three smallest complete shapes found
//! among them as ordinary Rust (no addresses, no numbered callee slots,
//! no global state, no dependence on pointer width) and restates each
//! verified 32-bit routine with behaviour in it as a method on its
//! owning type.
//!
//! - [`weighted`]: the weighted candidate picker: sixteen weights with
//!   cached result slots and an entry count, drawn by a random threshold
//!   over prefix sums.
//! - [`react`]: the facing reaction code: a ped's direction row and
//!   position against a task object's position, classified to one of six
//!   codes through a state probe and a counter gate.
//! - [`state`]: the shared ped-task state block: two 3-vectors, three
//!   biases, a gate reference, a range bound and a flag byte, built from
//!   one object's eight floats and consumed vector by vector.
//!
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results and every
//! effect (see the `lf-pedstaskdiff` test crate). Nothing here is verified
//! by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per routine in [`registry`].

#![forbid(unsafe_code)]

mod react;
mod state;
mod weighted;

pub mod registry;

pub use react::{FacingQuery, PedState, ReactTuning, CODE_FAR_HI, CODE_FAR_LO, CODE_MID_HI, CODE_MID_LO, CODE_NEAR_HI, CODE_NEAR_LO};
pub use state::{
    Check, Enumerate, GATE_LIMIT, HALF, NEG_HALF, OBJ_ID_VALUE, ObjTag, TaskFloats, TaskStamp,
    TaskStateBlock, TaskTarget, TaskVec, VTask, Worker,
};
pub use weighted::{PickerFill, PickerRand, WeightedPicker, EMPTY, MAX_ENTRIES, SCALE};
