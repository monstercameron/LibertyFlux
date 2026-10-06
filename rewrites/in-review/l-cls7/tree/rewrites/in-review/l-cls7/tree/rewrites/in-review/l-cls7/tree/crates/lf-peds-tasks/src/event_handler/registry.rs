//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit method of the event handler class.
//! `Proven` means the method is restated on [`EventHandler`](crate::event_handler::EventHandler)
//! and the differential test crate ran it against its verified rewrite on
//! the same generated inputs, comparing results and every effect, with a
//! deliberately wrong lift caught alongside. Counts below come from this
//! table.

/// Lift state of one verified method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on the handler type and proven against its rewrite.
    Proven,
    /// Restated but not proven (no row should stay here: everything
    /// lifted in this module is proven).
    Lifted,
    /// Not lifted, for the stated reason.
    Missing,
}

/// One verified method's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// Handler class; every row here is `"EventHandler"`.
    pub class: &'static str,
    /// Slot or method name in the 32-bit form, e.g. `"vf37"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the method is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified method of the handler class, in slot order.
pub const ROWS: &[Row] = &[
    // Lifecyle: destructors through callee slots (Drop covers them).
    Row {
        class: "EventHandler",
        method: "dtor",
        state: State::Missing,
        narrows: &[
            "scalar destructor through 2 callee slots and a class-table global: Drop covers it once the member teardown lifts",
        ],
    },
    Row {
        class: "EventHandler",
        method: "deleting_dtor",
        state: State::Missing,
        narrows: &[
            "deleting destructor (runs the scalar one, frees on flag bit 0): Drop covers it",
        ],
    },
    Row {
        class: "EventHandler",
        method: "dtor_2",
        state: State::Missing,
        narrows: &[
            "second destructor variant through 2 callee slots and class-table globals: Drop covers it",
        ],
    },
    Row {
        class: "EventHandler",
        method: "dtor_3",
        state: State::Missing,
        narrows: &[
            "third destructor variant through 2 callee slots and a class-table global: Drop covers it",
        ],
    },
    // The factory-pair slots: each reads the shared manager word, asks
    // it for a handler, and converts one request through it. The manager
    // travels as FactoryState, the two calls as the TaskFactory trait;
    // the proof scripts both stubs and compares slot and arguments in
    // order.
    Row {
        class: "EventHandler",
        method: "vf14",
        state: State::Proven,
        narrows: &[
            "the staged word folds the event address's bits: the lift computes it from the event cookie, which is the address on the 32-bit side",
            "the conversion's trailing zero pad word is pinned by the proof, not modelled in the request",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf54",
        state: State::Proven,
        narrows: &[
            "the pass-through answer (the event itself) and the conversion share one answer enum, mapped to words by the proof",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf55",
        state: State::Proven,
        narrows: &[
            "the kind-word answers and the conversion share one answer enum, mapped to words by the proof",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf67",
        state: State::Proven,
        narrows: &[
            "shares its lift with vf68 (one method, one fixed code each): the proof pins each slot's own code",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf68",
        state: State::Proven,
        narrows: &[
            "shares its lift with vf67 (one method, one fixed code each): the proof pins each slot's own code",
        ],
    },
    // Proven: the five call-free slots with behaviour in them.
    Row {
        class: "EventHandler",
        method: "vf29",
        state: State::Proven,
        narrows: &[
            "the event's clone call travels as the EventSource trait: the proof plants a stub in a fake event table and compares calls, answers and every stored word",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf31",
        state: State::Proven,
        narrows: &[
            "the constant-zero answer is not modelled (the lift returns nothing and the proof pins the zero)",
            "the self-dispatch call travels as the EventDispatch trait: the proof plants a stub in a fake handler table and compares calls and every stored word",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf34",
        state: State::Proven,
        narrows: &[
            "the payload word narrows to its presence (null or not)",
            "the constant-zero answer is not modelled (pinned by the proof)",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf36",
        state: State::Proven,
        narrows: &[
            "the constant-zero answer is not modelled (pinned by the proof)",
            "the event's poll calls travel as the EventSource trait: the proof scripts stub answers and compares the call count and order",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf37",
        state: State::Proven,
        narrows: &[],
    },
    // Proven: the refresh slots. Each reads the shared manager word,
    // asks it for a handler, and converts event words through it; the
    // manager travels as FactoryState, the two calls as the TaskFactory
    // trait, and the proof scripts both stubs and compares slot and
    // arguments in order.
    Row {
        class: "EventHandler",
        method: "vf21",
        state: State::Proven,
        narrows: &["the seen byte travels as a flag the proof maps onto the event blob"],
    },
    Row {
        class: "EventHandler",
        method: "vf23",
        state: State::Proven,
        narrows: &[
            "the conversion's trailing zero word is pinned by the proof, not modelled in the request",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf28",
        state: State::Proven,
        narrows: &[
            "the owner word must be live (the slot faults on null): the lift takes it as a non-optional handle",
            "the flag byte travels as a value the proof maps onto an owner blob",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf71",
        state: State::Proven,
        narrows: &[
            "the build's fixed all-ones word is pinned by the proof, not modelled in the request",
            "the allocator/builder pair shares the factory trait: the lookup allocates, the conversion builds",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf73",
        state: State::Proven,
        narrows: &[
            "the float word travels as f32 and is compared bitwise",
            "the conversion's trailing zero word is pinned by the proof, not modelled in the request",
            "the flag byte travels as a value the proof maps onto an owner blob",
            "the null-owner zero answer is not modelled (pinned by the proof)",
        ],
    },
    // Everything else reaches callees and lifts once its collaborator
    // traits exist. Counts are callee slots per rewrite (Verified: read
    // off the tracked rewrite files).
    Row {
        class: "EventHandler",
        method: "vf11",
        state: State::Missing,
        narrows: &["11 callee slots, a virtual call and shared globals"],
    },
    Row {
        class: "EventHandler",
        method: "vf13",
        state: State::Missing,
        narrows: &["19 callee slots, virtual calls and shared globals"],
    },
    Row {
        class: "EventHandler",
        method: "vf15",
        state: State::Missing,
        narrows: &["12 callee slots, a virtual call and a shared global"],
    },
    Row {
        class: "EventHandler",
        method: "vf18",
        state: State::Missing,
        narrows: &["19 callee slots, virtual calls and shared globals"],
    },
    Row {
        class: "EventHandler",
        method: "vf19",
        state: State::Missing,
        narrows: &["20 callee slots, a virtual call and shared globals"],
    },
    Row {
        class: "EventHandler",
        method: "vf24",
        state: State::Missing,
        narrows: &["7 callee slots and a shared global"],
    },
    Row {
        class: "EventHandler",
        method: "vf25",
        state: State::Proven,
        narrows: &[
            "the kind probes and readiness check travel as EventSource methods: the proof plants a stub in a fake event table and scripts the check",
            "the two-stage lookup travels as the StagedLookup trait: the proof scripts both stages and compares slots and arguments in order",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf27",
        state: State::Missing,
        narrows: &["6 callee slots and a shared global"],
    },
    Row {
        class: "EventHandler",
        method: "vf32",
        state: State::Missing,
        narrows: &["18 callee slots, a virtual call and shared globals"],
    },
    Row {
        class: "EventHandler",
        method: "vf33",
        state: State::Missing,
        narrows: &["7 callee slots and a shared global"],
    },
    Row {
        class: "EventHandler",
        method: "vf35",
        state: State::Proven,
        narrows: &[
            "the constant-zero answer is not modelled (pinned by the proof)",
            "the event block travels as its byte offset: the proof maps it onto the event address for the build and dispatch calls",
            "the two float globals travel as RouteInput floats and are compared bitwise",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf39",
        state: State::Missing,
        narrows: &["22 callee slots and shared globals"],
    },
    Row {
        class: "EventHandler",
        method: "vf41",
        state: State::Proven,
        narrows: &[
            "the owner word must be live (the slot faults on null): the lift takes it as a non-optional handle",
            "the ready probe and follow-up check travel as the SettleStatus trait: only their low bytes are tested",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf43",
        state: State::Missing,
        narrows: &["4 callee slots, three virtual calls and a shared global"],
    },
    Row {
        class: "EventHandler",
        method: "vf45",
        state: State::Missing,
        narrows: &["19 callee slots, a virtual call and shared globals"],
    },
    Row {
        class: "EventHandler",
        method: "vf46",
        state: State::Missing,
        narrows: &["32 callee slots, three virtual calls and shared globals"],
    },
    Row {
        class: "EventHandler",
        method: "vf47",
        state: State::Missing,
        narrows: &["9 callee slots, a virtual call and a shared global"],
    },
    Row {
        class: "EventHandler",
        method: "vf48",
        state: State::Missing,
        narrows: &["6 callee slots, a virtual call and a shared global"],
    },
    Row {
        class: "EventHandler",
        method: "vf50",
        state: State::Missing,
        narrows: &["9 callee slots and a shared global"],
    },
    Row {
        class: "EventHandler",
        method: "vf51",
        state: State::Missing,
        narrows: &["4 callee slots, three virtual calls and shared globals"],
    },
    Row {
        class: "EventHandler",
        method: "vf52",
        state: State::Missing,
        narrows: &["16 callee slots, two virtual calls and shared globals"],
    },
    Row {
        class: "EventHandler",
        method: "vf53",
        state: State::Missing,
        narrows: &["9 callee slots, three chained virtual calls and shared globals"],
    },
    Row {
        class: "EventHandler",
        method: "vf56",
        state: State::Missing,
        narrows: &["5 callee slots and a shared global"],
    },
    Row {
        class: "EventHandler",
        method: "vf57",
        state: State::Missing,
        narrows: &["9 callee slots, three virtual calls and a shared global"],
    },
    Row {
        class: "EventHandler",
        method: "vf58",
        state: State::Missing,
        narrows: &["4 callee slots, a virtual call and shared globals"],
    },
    Row {
        class: "EventHandler",
        method: "vf60",
        state: State::Missing,
        narrows: &["7 callee slots and a shared global"],
    },
    Row {
        class: "EventHandler",
        method: "vf61",
        state: State::Proven,
        narrows: &[
            "the owner word must be live (the slot faults on null): the lift takes it as a non-optional handle",
            "the probe status word travels as a value the proof maps onto a probe blob",
            "the registry check and release travel as the ProbeRegistry trait: the proof scripts both and compares slots and arguments in order",
            "the conversion's trailing zero pad word is pinned by the proof, not modelled in the request",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf62",
        state: State::Missing,
        narrows: &["4 callee slots and shared globals"],
    },
    Row {
        class: "EventHandler",
        method: "vf63",
        state: State::Proven,
        narrows: &[
            "the scalar call runs before the lookup on every path, as in the original",
            "the vector base travels as its byte offset: the proof maps it onto the event address",
            "the scalar and float words travel as f32 and are compared bitwise",
            "the conversion's trailing zero pad word is pinned by the proof, not modelled in the request",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf64",
        state: State::Proven,
        narrows: &[
            "the scalar call travels as ScalarEval::eval_block: the proof scripts its answer bitwise and compares all seven arguments",
            "the event-relative pointers in both calls are derived from the event handle by the proof",
            "the sibling converter choice travels as the request's second flag: the proof asserts which stub fired",
        ],
    },
    Row {
        class: "EventHandler",
        method: "vf65",
        state: State::Missing,
        narrows: &["10 callee slots and a shared global"],
    },
    Row {
        class: "EventHandler",
        method: "vf66",
        state: State::Missing,
        narrows: &["8 callee slots, a virtual call and a shared global"],
    },
    Row {
        class: "EventHandler",
        method: "vf69",
        state: State::Missing,
        narrows: &["14 callee slots and a shared global"],
    },
    Row {
        class: "EventHandler",
        method: "vf72",
        state: State::Missing,
        narrows: &["5 callee slots and shared globals"],
    },
    Row {
        class: "EventHandler",
        method: "vf74",
        state: State::Missing,
        narrows: &[
            "7 callee slots and shared globals (speech request, scripted-action and blend-target paths with float arithmetic)",
        ],
    },
];

/// Number of rows in each state.
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
