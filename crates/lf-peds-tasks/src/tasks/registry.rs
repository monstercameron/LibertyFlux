//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit method of the six task classes.
//! `Proven` means the method is restated on its task type and the
//! differential test crate ran it against its verified rewrite on the
//! same generated inputs, comparing results and every effect, with a
//! deliberately wrong lift caught alongside. Counts below come from this
//! table.

/// Lift state of one verified method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on the task type and proven against its rewrite.
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
    /// Task class short name, e.g. `"Duck"`.
    pub class: &'static str,
    /// Constructor, slot or method name in the 32-bit form, e.g. `"vf17"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the method is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified method of the six task classes, grouped by class.
pub const ROWS: &[Row] = &[
    // HitResponse: the kind word and its three proven methods.
    Row {
        class: "HitResponse",
        method: "ctor",
        state: State::Proven,
        narrows: &[
            "the class table stamp is pinned by the proof, not modelled",
            "the base-constructor call travels as the HitBase trait: the proof plants a stub and compares the call and the blob address it receives",
        ],
    },
    Row {
        class: "HitResponse",
        method: "deleting_dtor",
        state: State::Missing,
        narrows: &[
            "scalar deleting destructor (runs the destructor, frees through the task pool on flag bit 0): Drop covers it",
        ],
    },
    Row {
        class: "HitResponse",
        method: "vf1",
        state: State::Proven,
        narrows: &[
            "the allocator and the initialiser travel as the HitPool trait: the proof scripts both stubs and compares slot and arguments in order",
        ],
    },
    Row {
        class: "HitResponse",
        method: "vf19",
        state: State::Proven,
        narrows: &[
            "the four case callees share one trait method: the proof pins the kind-to-slot mapping",
            "the lookup travels as the HitStart trait: the proof scripts the stub and compares calls and the untouched blob",
        ],
    },
    // SeekCover: six heavy methods, none lifted in this lane.
    Row {
        class: "SeekCover",
        method: "ctor",
        state: State::Missing,
        narrows: &[
            "full constructor through 4 callee slots (base, two member initialisers on interior pointers, tail initialiser), vectors, mode flags and a latch bit: not lifted in this lane",
        ],
    },
    Row {
        class: "SeekCover",
        method: "ctor_2",
        state: State::Missing,
        narrows: &[
            "second constructor through 5 callee slots with a cover-table row query path: not lifted in this lane",
        ],
    },
    Row {
        class: "SeekCover",
        method: "ctor_3",
        state: State::Missing,
        narrows: &[
            "shorter constructor variant through 4 callee slots (zeroed second vector, cleared low flag bit): not lifted in this lane",
        ],
    },
    Row {
        class: "SeekCover",
        method: "dtor",
        state: State::Missing,
        narrows: &[
            "two member destructors, two slot releases and a tail to the base through 4 callee slots: Drop covers it",
        ],
    },
    Row {
        class: "SeekCover",
        method: "vf1",
        state: State::Missing,
        narrows: &[
            "subtask router through 4 callee slots with interior field pointers and a flag sync that faults on a null pick: not lifted in this lane",
        ],
    },
    Row {
        class: "SeekCover",
        method: "vf21",
        state: State::Missing,
        narrows: &[
            "node allocator through 2 callee slots with a table stamp and zeroed tail: not lifted in this lane",
        ],
    },
    // ShockingEventFlee: the spawner and the periodic update are proven.
    Row {
        class: "ShockingEventFlee",
        method: "ctor",
        state: State::Missing,
        narrows: &[
            "a single flag-byte clear behind the base constructor and table stamp: no liftable behaviour beyond the base call",
        ],
    },
    Row {
        class: "ShockingEventFlee",
        method: "vf0",
        state: State::Missing,
        narrows: &[
            "scalar deleting destructor (runs the destructor, frees through the memory manager on flag bit 0): Drop covers it",
        ],
    },
    Row {
        class: "ShockingEventFlee",
        method: "vf1",
        state: State::Missing,
        narrows: &[
            "clone through 2 callee slots plus a direct state-byte store into the opaquely built clone, faulting on a null allocation: no clean lift shape in this lane",
        ],
    },
    Row {
        class: "ShockingEventFlee",
        method: "vf20",
        state: State::Proven,
        narrows: &[
            "the probe, check and dispatch calls travel as the FleePoll trait: the proof plants a stub in a fake task table for the check slot and scripts the direct callees, comparing slot and arguments in order",
            "the probe and dispatch calls' task address and the check call's (1, 0) words are pinned by the proof, not modelled",
            "the length threshold travels as a value the proof maps onto the threshold global",
        ],
    },
    Row {
        class: "ShockingEventFlee",
        method: "vf19",
        state: State::Missing,
        narrows: &[
            "flee-task builder through 12 callee slots over 6 globals (task table, seed word, clock pair, manager, rate word): not lifted in this lane",
        ],
    },
    Row {
        class: "ShockingEventFlee",
        method: "vf18",
        state: State::Missing,
        narrows: &[
            "reaction update through 5 callee slots behind entity kind gates: not lifted in this lane",
        ],
    },
    Row {
        class: "ShockingEventFlee",
        method: "vf7",
        state: State::Proven,
        narrows: &[
            "the allocator and the two constructor forms travel as the FleeSpawn trait: the proof scripts all three stubs and compares slot and arguments in order",
            "the constructors' (1, 1) words are pinned by the proof, not modelled",
            "the vector form's spilled position pointer is snapped by the proof (3 words) and compared bitwise",
            "the length threshold travels as a value the proof maps onto the threshold global; the manager word travels as a value mapped onto this method's own manager global",
        ],
    },
    // ShockingEventGoto: the clone slot and the periodic update are proven.
    Row {
        class: "ShockingEventGoto",
        method: "ctor",
        state: State::Missing,
        narrows: &[
            "base constructor, table stamp, cleared trailing words and a radius query through 2 callee slots: not lifted in this lane",
        ],
    },
    Row {
        class: "ShockingEventGoto",
        method: "vf0",
        state: State::Missing,
        narrows: &[
            "scalar deleting destructor (runs the destructor, frees through the memory manager on flag bit 0): Drop covers it",
        ],
    },
    Row {
        class: "ShockingEventGoto",
        method: "vf1",
        state: State::Proven,
        narrows: &[
            "the allocator and the copy constructor travel as the GotoPool trait: the proof scripts both stubs and compares slot and arguments in order",
            "the copy constructor receives the member's address: the lift passes the member word and the proof pins the address",
        ],
    },
    Row {
        class: "ShockingEventGoto",
        method: "vf20",
        state: State::Proven,
        narrows: &[
            "the probe, fallback and dispatch calls travel as the GotoPoll trait: the proof scripts the three stubs and compares slot and arguments in order",
            "the probe and dispatch calls' task address and the fallback call's table, blend and zero words are pinned by the proof, not modelled",
            "the subtask's check sequence (gate bit, slot call, mark set) travels as one trait call: the proof plants a stub in a fake subtask table and compares the call, the scripted verdict and the subtask blob",
            "the tick and the length threshold travel as values the proof maps onto the tick and threshold globals",
            "a null subtask panics where the original faults (every path but the probe-keeps path); the proof scripts live subtasks except there",
        ],
    },
    Row {
        class: "ShockingEventGoto",
        method: "vf19",
        state: State::Missing,
        narrows: &[
            "goto-target picker with float hashing and three child builders through 11 callee slots: not lifted in this lane",
        ],
    },
    // Duck: three proven methods; the deleting destructor is Drop.
    Row {
        class: "Duck",
        method: "vf0",
        state: State::Missing,
        narrows: &[
            "scalar deleting destructor (runs the destructor, frees through the game allocator on flag bit 0): Drop covers it",
        ],
    },
    Row {
        class: "Duck",
        method: "vf1",
        state: State::Proven,
        narrows: &[
            "the allocator and the copy constructor travel as the DuckPool trait: the proof scripts both stubs and compares slot and arguments in order",
            "the level word travels as i16 and is compared sign-extended",
        ],
    },
    Row {
        class: "Duck",
        method: "vf5",
        state: State::Proven,
        narrows: &[
            "the announce call's constant words (0, all-ones) are pinned by the proof, not modelled",
            "the query and trigger calls travel as the DuckEvent trait: the proof plants stubs in a fake object table and compares calls, answers and every stored word",
        ],
    },
    Row {
        class: "Duck",
        method: "vf17",
        state: State::Proven,
        narrows: &[
            "the timeout tick and the sample threshold travel as values the proof maps onto the tick and float globals",
            "the announce call's trailing all-ones word, the sustain call's task address and trailing zeros, and the finish check's (1, 0) words are pinned by the proof, not modelled",
            "the ped sample and announce calls travel as the DuckPedSide trait and the sustain, elapsed and finish calls as the DuckTaskSide trait: the proof scripts stubs and compares slot and arguments in order",
        ],
    },
    // ShakeFist: two proven methods; the destructors are Drop.
    Row {
        class: "ShakeFist",
        method: "dtor",
        state: State::Missing,
        narrows: &[
            "member cleanup through 3 callee slots, one taking an interior slot pointer, then a tail to the base destructor: Drop covers it",
        ],
    },
    Row {
        class: "ShakeFist",
        method: "vf0",
        state: State::Missing,
        narrows: &[
            "scalar deleting destructor (runs the destructor, frees through the game allocator on flag bit 0): Drop covers it",
        ],
    },
    Row {
        class: "ShakeFist",
        method: "vf1",
        state: State::Proven,
        narrows: &[
            "the allocator and the copy constructor travel as the FistPool trait: the proof scripts both stubs and compares slot and arguments in order",
        ],
    },
    Row {
        class: "ShakeFist",
        method: "vf5",
        state: State::Proven,
        narrows: &[
            "the release call's task-address argument is pinned by the proof (the stub receives the blob address)",
            "the damp call takes the slot by mutable reference: the proof's stub clears it through a side channel on scripted cases and both sides skip the release",
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
