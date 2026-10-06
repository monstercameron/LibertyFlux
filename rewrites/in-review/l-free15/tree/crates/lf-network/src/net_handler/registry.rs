//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified routine of the lane's `net_handler_*` list: the
//! twelve handler-swap instances, the four registrar thunks, and the two
//! session-manager methods. `Proven` means the routine is restated as
//! [`HandlerSlots::swap`](crate::net_handler::HandlerSlots::swap) and the
//! differential test crate ran it against its verified rewrite on the
//! same generated inputs, comparing the answer and every written word,
//! with a deliberately wrong lift caught alongside. Counts below come
//! from this table.
//!
//! Later lanes add one section each for the session manager and whatever
//! else the registrar thunks' callee turns out to own.

/// Lift state of one verified routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on its owning type and proven against its rewrite.
    Proven,
    /// Restated but not proven (no row should stay here: everything
    /// lifted in this module is proven).
    Lifted,
    /// Not lifted, for the stated reason.
    Missing,
}

/// One verified routine's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// Naming-lane name of the verified routine, e.g. `"net_handler_swap_600"`.
    pub func: &'static str,
    /// Lifted method, e.g. `"HandlerSlots::swap"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the routine is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified routine of the lane's list.
pub const ROWS: &[Row] = &[
    // The twelve handler-swap instances: one routine, one generic method,
    // proven once per instance. No narrowing: the return word, the shared
    // slot and the save cell are all compared, and handlers stay opaque
    // cookies on both sides.
    Row {
        func: "net_handler_swap_600",
        method: "HandlerSlots::swap",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        func: "net_handler_swap_620",
        method: "HandlerSlots::swap",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        func: "net_handler_swap_640",
        method: "HandlerSlots::swap",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        func: "net_handler_swap_660",
        method: "HandlerSlots::swap",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        func: "net_handler_swap_680",
        method: "HandlerSlots::swap",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        func: "net_handler_swap_6a0",
        method: "HandlerSlots::swap",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        func: "net_handler_swap_6c0",
        method: "HandlerSlots::swap",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        func: "net_handler_swap_6e0",
        method: "HandlerSlots::swap",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        func: "net_handler_swap_700",
        method: "HandlerSlots::swap",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        func: "net_handler_swap_720",
        method: "HandlerSlots::swap",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        func: "net_handler_swap_740",
        method: "HandlerSlots::swap",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        func: "net_handler_swap_760",
        method: "HandlerSlots::swap",
        state: State::Proven,
        narrows: &[],
    },
    // The four registrar thunks: each pushes its handler address and
    // forwards to the shared registrar, answering its 0/1. A routine
    // whose whole content is one address plus one forwarded call carries
    // no behaviour to lift.
    Row {
        func: "net_handler_register_540",
        method: "-",
        state: State::Missing,
        narrows: &["forward-only thunk: one pushed address, one registrar call, no behaviour"],
    },
    Row {
        func: "net_handler_register_550",
        method: "-",
        state: State::Missing,
        narrows: &["forward-only thunk: one pushed address, one registrar call, no behaviour"],
    },
    Row {
        func: "net_handler_register_560",
        method: "-",
        state: State::Missing,
        narrows: &["forward-only thunk: one pushed address, one registrar call, no behaviour"],
    },
    Row {
        func: "net_handler_register_5d0",
        method: "-",
        state: State::Missing,
        narrows: &["forward-only thunk: one pushed address, one registrar call, no behaviour"],
    },
    // The two session-manager methods: not reached. Each needs its
    // callees as traits and the session object's native struct first.
    Row {
        func: "net_session_join_result_apply (proposed)",
        method: "-",
        state: State::Missing,
        narrows: &["not reached: 13 callees over the session manager plus a signed table walk"],
    },
    Row {
        func: "net_handler_large (proposed)",
        method: "-",
        state: State::Missing,
        narrows: &["not reached: 27 callees, a virtual kind dispatch and a clock read"],
    },
];

/// Counts of rows by state: `(proven, lifted, missing)`.
#[must_use]
pub const fn counts() -> (usize, usize, usize) {
    let mut proven = 0;
    let mut lifted = 0;
    let mut missing = 0;
    let mut i = 0;
    while i < ROWS.len() {
        match ROWS[i].state {
            State::Proven => proven += 1,
            State::Lifted => lifted += 1,
            State::Missing => missing += 1,
        }
        i += 1;
    }
    (proven, lifted, missing)
}
