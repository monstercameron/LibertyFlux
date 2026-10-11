//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit routine of the group, in structure order.
//! `Proven` means the routine is restated on its owning type and the
//! differential test crate ran it against its verified rewrite on the same
//! generated inputs, comparing results, every written byte and every
//! collaborator call in order, with a deliberately wrong lift caught
//! alongside. Counts below come from this table.

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
    /// Owning structure, e.g. `"HandleTable"`.
    pub structure: &'static str,
    /// Proposed routine name, e.g. `"input_slot_find_free"`.
    pub routine: &'static str,
    /// Lifted method, e.g. `"SlotStore::find_free"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the routine is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified routine of the group, in structure order.
pub const ROWS: &[Row] = &[
    // The handle table.
    Row {
        structure: "HandleTable",
        routine: "input_slot_find_free",
        method: "SlotStore::find_free",
        state: State::Proven,
        narrows: &[
            "flag word narrows to its low byte (big or small install)",
            "start at or past the table answers all-ones (proven against planted guard words; a start of all-ones with a non-null word before the table wraps the original's scan to slot 0, which is out of domain)",
            "allocated blocks travel as collaborator cookies; the constructor's answer address is unobservable (no routine returns it)",
        ],
    },
    Row {
        structure: "HandleTable",
        routine: "input_slot_notify_kind",
        method: "SlotStore::notify_kind",
        state: State::Proven,
        narrows: &[
            "sink objects travel as a three-way tag; missing or past-the-table indexes panic where the original reads wild memory or faults",
        ],
    },
    Row {
        structure: "HandleTable",
        routine: "input_slot_flag_byte",
        method: "SlotStore::flag_byte",
        state: State::Proven,
        narrows: &[
            "zero-extended byte answer travels as u8",
            "one resolve routine shared with the mode word (the rewrites differ in read width only); each width proven separately",
            "past-the-table or null slots panic where the original reads wild memory or faults; small records panic where the original over-reads the heap",
        ],
    },
    Row {
        structure: "HandleTable",
        routine: "input_slot_mode_word",
        method: "SlotStore::mode_word",
        state: State::Proven,
        narrows: &[
            "one resolve routine shared with the flag byte; each width proven separately",
            "past-the-table or null slots panic where the original reads wild memory or faults; small records panic where the original over-reads the heap",
        ],
    },
    Row {
        structure: "HandleTable",
        routine: "input_slot_announce",
        method: "SlotStore::announce",
        state: State::Proven,
        narrows: &[
            "payload address narrows to the payload block (the proof rebuilds the address per case and compares it)",
            "formatter text travels as an opaque word; the sink object is a collaborator call",
            "past-the-table or null slots panic where the original reads wild memory or faults; small records panic where the original over-reads the heap",
        ],
    },
    Row {
        structure: "HandleTable",
        routine: "input_slot_destroy",
        method: "SlotStore::destroy",
        state: State::Proven,
        narrows: &[
            "by-handle word narrows to bool (low byte); the low-byte result residue narrows to its meaning (the proof rebuilds the full word per case)",
            "thread entry travels as its owned flag (production wiring fetches it behind the platform thread-local trait)",
            "past-the-table indexes panic where the original reads and clears wild memory",
        ],
    },
    Row {
        structure: "HandleTable",
        routine: "input_slot_configure",
        method: "SlotStore::configure",
        state: State::Missing,
        narrows: &[
            "not lifted: a 34-callee sampler machine (frame fills, resolve, combine, field setters, finish) needing a sample-source trait design first",
        ],
    },
    Row {
        structure: "HandleTable",
        routine: "input_slot_probe",
        method: "SlotStore::probe",
        state: State::Missing,
        narrows: &[
            "not lifted: feeds record fields to (pointer, length) sinks with frame-shape snapshots needing a sink-trait design first",
        ],
    },
    // The allocation registry.
    Row {
        structure: "Registry",
        routine: "input_slot_alloc",
        method: "SlotRegistry::alloc_slot",
        state: State::Proven,
        narrows: &[
            "failed-allocation null cells travel as the failed counter values",
            "allocated blocks travel as collaborator cookies; the initialiser's answer address is unobservable (no routine returns it)",
        ],
    },
    // The key table.
    Row {
        structure: "KeyTable",
        routine: "input_slot_index_by_key",
        method: "KeyTable::find",
        state: State::Proven,
        narrows: &["object key word narrows to its value"],
    },
    // The fixed-stride slot blocks.
    Row {
        structure: "StrideBlocks",
        routine: "input_slot_copy",
        method: "SlotBlocks::copy",
        state: State::Missing,
        narrows: &[
            "not lifted: two block copies through the shared copier needing a block-memory model first",
        ],
    },
    Row {
        structure: "StrideBlocks",
        routine: "input_slot_touch",
        method: "SlotBlocks::touch",
        state: State::Missing,
        narrows: &[
            "not lifted: refresher dispatch plus level-cell bumps over stride-0x110 records needing a block-memory model first",
        ],
    },
    // The dispatch object.
    Row {
        structure: "Dispatch",
        routine: "input_slot_dispatch",
        method: "Dispatch::dispatch",
        state: State::Missing,
        narrows: &[
            "not lifted: flag-maze dispatch with registration, key bumping and up to six sink calls needing a sink-trait design first",
        ],
    },
    // The controller resolve-and-emit.
    Row {
        structure: "ResolveEmit",
        routine: "input_slot_resolve_and_emit",
        method: "ResolveEmit::resolve_and_emit",
        state: State::Missing,
        narrows: &[
            "not lifted: slot-array walk with float scaling and a 19-argument emit call needing a slot-array and emit-trait design first",
        ],
    },
];

/// Number of rows in each state, in table order.
#[must_use]
pub fn counts() -> (usize, usize, usize) {
    let mut proven = 0;
    let mut lifted = 0;
    let mut missing = 0;
    for row in ROWS {
        match row.state {
            State::Proven => proven += 1,
            State::Lifted => lifted += 1,
            State::Missing => missing += 1,
        }
    }
    (proven, lifted, missing)
}
