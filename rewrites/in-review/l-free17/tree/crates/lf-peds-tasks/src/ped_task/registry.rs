//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit routine of the covered structures. `Proven`
//! means the routine is restated on its owning type and the differential
//! test crate ran it against its verified rewrite on the same generated
//! inputs, comparing results and every effect, with a deliberately wrong
//! lift caught alongside. Counts below come from this table.

/// Lift state of one verified routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on the owning type and proven against its rewrite.
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
    /// Owning structure short name, e.g. `"PoseVolume"`.
    pub class: &'static str,
    /// Routine name in the 32-bit form, e.g. `"pose_transform"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the routine is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified routine of the covered structures, grouped by structure.
pub const ROWS: &[Row] = &[
    // PoseVolume: the pose trio. All three verified routines of this
    // object in the group are lifted; the shape is complete.
    Row {
        class: "PoseVolume",
        method: "pose_transform",
        state: State::Proven,
        narrows: &[
            "the w slots are zero: the original copied an uninitialised scratch word there and the rewrite pins it to zero, which is what the lift matches",
            "the link build/fetch pair travels as the LinkMatrix trait: the proof scripts the stubs, compares call order and count, and asserts the stub addresses against the planted link and matrix",
            "the second build site is unreachable (nothing runs between the two reads, and a null after the first build faults at the first vector): mirrored in the lift, not exercised",
            "a build that leaves no matrix faults in the original and panics in the lift (host-proven)",
        ],
    },
    Row {
        class: "PoseVolume",
        method: "pose_blend",
        state: State::Proven,
        narrows: &[
            "the fill slot (thiscall on the object, inferred to be the transform above from its identical signature, object and lane) travels as the PoseFill trait: the proof scripts both sides and compares the call",
            "the out2 address answer narrows to the computed values: the proof compares the written words",
            "the one-half factor is a caller-supplied value for the relocated tuning word",
        ],
    },
    Row {
        class: "PoseVolume",
        method: "cone_test",
        state: State::Proven,
        narrows: &[
            "the fill slot travels as the PoseFill trait, as in the blend",
            "the angle, cosine-like and sine-like callees travel as ConeSolvers methods: the proof scripts answers on both sides and compares arguments bit for bit",
            "the four normaliser callees share one trait method, told apart by NormSlot: the proof pins the slot order and the count immediate",
            "the six tuning words (half, one, half-pi, tau, two bit masks) are caller-supplied values for the relocated words",
            "angle-wrap loops are exercised only over sane angles and a positive tau; an infinite angle or a non-positive tau would not terminate, as in the original",
        ],
    },
    // BuildManagers: the kind-0x11 builder pair. One generic method over
    // the trailing words, proven once per arity.
    Row {
        class: "BuildManagers",
        method: "build4",
        state: State::Proven,
        narrows: &[
            "the ped slot read (task pointer, flag byte) travels as the ped_task context call: the proof plants the ped image and scripts the same outcome",
            "a null ped lookup faults in the original and panics in the lift (host-proven)",
            "the trailing kind word is passed explicitly so the proof compares it; the four-word cdecl shape is pinned by the stub",
            "task answers travel as opaque handles; the proof compares their raw words with the rewrite's answers",
        ],
    },
    Row {
        class: "BuildManagers",
        method: "build5",
        state: State::Proven,
        narrows: &["as build4, with the five-word cdecl shape pinned by the stub"],
    },
    // ChainCloner: the entry-chain clone pair. One generic method over
    // the done-flag write, proven once per instance.
    Row {
        class: "ChainCloner",
        method: "clone_chain_a",
        state: State::Proven,
        narrows: &[
            "the chain lookup, makers, setters and direct stores travel as ChainClone methods: the proof scripts entries and products, compares every call in order with its arguments, and compares the product image words the rewrite writes",
            "entry and product addresses travel as opaque handles; the proof compares their raw words with the stub answers",
            "the constant 0 answer narrows to ()",
        ],
    },
    Row {
        class: "ChainCloner",
        method: "clone_chain_b",
        state: State::Proven,
        narrows: &[
            "as clone_chain_a, with no done-flag write: the proof asserts mark_done is never called and the flag image word keeps its planted value",
        ],
    },
    // MoverPose: the mover trio. The pose and the blend state block are
    // the owned data; the family's callees and virtual methods travel as
    // MoverCallees.
    Row {
        class: "MoverPose",
        method: "lerp_position_heading",
        state: State::Proven,
        narrows: &[
            "the height setter (the state block's virtual slot) travels as MoverCallees::set_height; the proof checks that the rewrite passes its trailing padding word as zero, which the lift does not model",
            "the target pose and the state block are planted as images; the callees are assumed not to write the pose or the block, which the lift cannot observe through a shared reference",
            "the height passes through the slot as bits: a signalling NaN height would be quieted by the x87 return convention of a getter, so the proof's heights are quiet",
        ],
    },
    Row {
        class: "MoverPose",
        method: "drive_blend",
        state: State::Proven,
        narrows: &[
            "the blend into the state block (callee id 4 of the rewrite, same argument shape as the lerp routine) is an opaque MoverCallees::blend_into: the proof compares its call, arguments and count, not its effect; that it is the lerp routine is inferred from the shape and not established by a callee map",
            "the setup, readiness, encode callees and the height setter travel as MoverCallees methods; the proof compares every call in order with its arguments",
            "the target is a handle, compared as a raw word (null is None)",
            "the callees are assumed not to write the pose or the state block",
        ],
    },
    // TaskKey: the id lookup. Its two lookups and the record's id word
    // travel as TaskIdTable methods.
    Row {
        class: "TaskKey",
        method: "id_or_default",
        state: State::Proven,
        narrows: &[
            "the key is read from the owner's key word by the caller; the proof plants it in the owner image and compares the owner image afterwards",
            "the two lookups travel as TaskIdTable::primary and fallback: the proof compares each call in order with its key argument and answer; the fallback is asked only when the primary answers zero",
            "the id word of the final record is a plain read through TaskIdTable::record_id; the proof plants the records and compares the id returned",
            "a null final record (both lookups answer zero) faults in the original on the id word; the proof never plants that case, and the lift does not model the fault",
        ],
    },
    // The task request gate. Its status probe travels as GateStatus; the
    // globals and the task's vtable word are read by the caller.
    Row {
        class: "(free function)",
        method: "admits",
        state: State::Proven,
        narrows: &[
            "the task's vtable flag word and the two global words are read by the caller and passed as GateWords; the proof plants them through the relocated table and the vtable image",
            "the candidate's state and sub-flag words are read by the caller and passed as GateCandidate; the proof compares the values read",
            "the status probe is the task's callee, travelling as GateStatus::status; the proof compares the task argument and the answer",
            "the gate writes nothing; the proof checks that the rewrite writes none of its images",
        ],
    },
    // SeekRecord: the seek record builder. No callees.
    Row {
        class: "SeekRecord",
        method: "seek_target_setup",
        state: State::Proven,
        narrows: &[
            "the mover's position block is reached through the owner's sub-object pointer; the caller resolves that chain and passes the block's values, and a null sub-object (which faults in the rewrite) is not modelled",
            "the goal is the target's position block, of which only the x and y words are read: the proof plants the rest of the block and checks it is not written",
            "the word the original copies from an uninitialised frame slot is pinned to zero by the rewrite's contract; the lift writes zero",
            "the mode word the routine stores beside the record (two) is returned as SEEK_MODE for the caller to store",
        ],
    },
    Row {
        class: "MoverPose",
        method: "commit_blend",
        state: State::Proven,
        narrows: &[
            "the flag and state notifications, the three decoders and the height getter travel as MoverCallees methods; the proof compares every call in order with its arguments and answers",
            "the state block's height getter is read after the state notification, as in the original; the callees are assumed not to write the pose or the block",
            "the flag byte keeps its other bits, as the rewrite does; only bit 0 is written by the family",
        ],
    },
    // EventTable: the 32-record task event table. The records are owned
    // by the table; the helpers travel as EventCallees and EventObjects.
    // Record arguments of the helpers are indices, and the proof rebuilds
    // the address the 32-bit rewrite passes.
    Row {
        class: "EventTable",
        method: "refresh_matching_deadlines",
        state: State::Proven,
        narrows: &[
            "the game clock word is read by the caller and passed as current_time; the proof plants it in the auxiliary window the rewrite reads",
            "the delay helper travels as EventCallees::delay_ticks; the proof compares each call in order with its task argument",
            "the callees are assumed not to write the table, which the lift cannot observe through its exclusive borrow",
            "the routine has no return value; none is compared",
        ],
    },
    Row {
        class: "EventTable",
        method: "release_lowest_deadline_record",
        state: State::Proven,
        narrows: &[
            "the selection-mode helper travels as EventCallees::selection_mode and is called on every run, after the scan, as in the original; only its low byte is read",
            "the released record is passed as its index; the proof rebuilds the record address the rewrite passes",
            "the callees are assumed not to write the table",
            "the routine has no return value; none is compared",
        ],
    },
    Row {
        class: "EventTable",
        method: "find_by_event",
        state: State::Proven,
        narrows: &[
            "the answer is the record index; the proof rebuilds the record address from it and compares it with the rewrite's answer, zero for none",
            "a zero key matches nothing, as the rewrite's early return does",
        ],
    },
    Row {
        class: "EventTable",
        method: "holds_object",
        state: State::Proven,
        narrows: &[
            "the answer is a bool; the proof compares the low byte the rewrite returns, since its upper return bits are not meaningful",
            "an index past the table never matches, as the rewrite's bound check does",
        ],
    },
    Row {
        class: "EventTable",
        method: "set_event_flag",
        state: State::Proven,
        narrows: &[
            "the routine returns the image address one past the scanned table, a fixed value of the original's loop; the lift does not return it and the proof does not compare it",
            "only the low byte of the flag is stored, as the rewrite does",
        ],
    },
    Row {
        class: "EventTable",
        method: "release_active",
        state: State::Proven,
        narrows: &["an instance of release_where: releases each active record"],
    },
    Row {
        class: "EventTable",
        method: "release_nonqueued",
        state: State::Proven,
        narrows: &["an instance of release_where: releases each active record whose kind is not the queued kind"],
    },
    Row {
        class: "EventTable",
        method: "replace_object",
        state: State::Proven,
        narrows: &[
            "the release and retain helpers (thiscall) travel as EventCallees::release_object and retain_object, and receive the record index where the original passes the address of the object word; the proof rebuilds that address",
            "the callees are assumed not to write the table; the store happens between the release and the retain, as in the original",
            "an index past the table is ignored, as the rewrite's bound check does",
            "the routine has no return value; none is compared",
        ],
    },
    Row {
        class: "EventTable",
        method: "check_nearby_event",
        state: State::Proven,
        narrows: &[
            "the target vector's three words are read once by the caller and passed as target; the rewrite reads them per event, and nothing writes them in between",
            "the event object's type byte, vector id and position are reads through EventObjects, which the implementation supplies (the position is inline or behind the event's pointer); the proof's implementation reads the same layout, and the proof compares the calls that follow from those reads",
            "the proximity helper's option (one), descriptor and trailing zero are supplied by the implementation of proximity_notify; the proof compares them in the call log",
            "the squared distance is single precision with the differences squared and summed in y, x, z order; the comparison is strict, so a NaN distance is not nearby",
            "the callees and the event objects are assumed not to write the table or the events",
            "the routine has no return value; none is compared",
        ],
    },
    Row {
        class: "EventTable",
        method: "ped_task_event_registration (forwarder)",
        state: State::Missing,
        narrows: &[
            "a single helper call with constant arguments and no branch or loop: nothing to lift, so the routine is left out",
        ],
    },
    Row {
        class: "(receiver, not the table)",
        method: "ped_task_select_event_entry",
        state: State::Missing,
        narrows: &[
            "reads a child object through the receiver and calls its virtual query and a resolver; a receiver shape, not the table, not lifted in this lane",
        ],
    },
    Row {
        class: "(receiver, not the table)",
        method: "ped_task_replace_owner_object",
        state: State::Missing,
        narrows: &[
            "stores an owner pointer at a fixed offset of the receiver and calls the old object's release and the deallocator; a receiver shape, not the table, not lifted in this lane",
        ],
    },    // TaskSlots: the task manager's two pointer arrays. The task interface
    // and the manager's preparatory methods travel as ManagerCallees.
    Row {
        class: "TaskSlots",
        method: "first_occupied",
        state: State::Proven,
        narrows: &[
            "the answer is the index of the first nonnull primary pointer, or the primary count when all are null; the rewrite reads the same five words and writes nothing",
        ],
    },
    Row {
        class: "TaskSlots",
        method: "contains",
        state: State::Proven,
        narrows: &[
            "the answer is a bool; the proof compares the low byte the rewrite returns",
            "the needle is compared with the six secondary words only, as the rewrite does",
        ],
    },
    Row {
        class: "TaskSlots",
        method: "request_bomb_task",
        state: State::Proven,
        narrows: &[
            "the task interface's two other stack words are zero in the rewrite; the lift passes only the kind, and the proof's recorder checks the zero words in the call log",
            "the interface's answer is not exposed by the rewrite, so none is compared",
        ],
    },
    Row {
        class: "TaskSlots",
        method: "request_first_bomb_task",
        state: State::Proven,
        narrows: &[
            "as request_bomb_task: the interface's two other stack words are zero, checked in the call log; its answer is not compared",
        ],
    },
    Row {
        class: "TaskSlots",
        method: "update_sequence",
        state: State::Proven,
        narrows: &[
            "the three methods travel as ManagerCallees; the proof compares their order and the manager handle each receives",
            "the answer is the final method's answer, returned unchanged",
        ],
    },
    Row {
        class: "TaskSlots",
        method: "drive_task_update (not lifted)",
        state: State::Missing,
        narrows: &[
            "reads and writes an embedded task record at a fixed offset of the manager and an update source of the game state, and reads the game clock; a different sub-object from the slot arrays, not lifted in this lane",
        ],
    },    // AudioTask: the three audio object slots of the task record and the
    // release that frees each. One generic method; each slot's routine is
    // an instance, proven against its own rewrite.
    Row {
        class: "AudioTask",
        method: "release_audio_task",
        state: State::Proven,
        narrows: &[
            "the primary-slot instance of release_slot; the three game globals are read by the caller and passed as ReleaseGlobals, and the proof plants them as relocated words",
            "the resource word and the cleanup and release helpers travel as AudioObjects; the proof compares every call in order with its arguments",
            "the slot is not cleared by the release, as in the original",
        ],
    },
    Row {
        class: "AudioTask",
        method: "release_radio_task",
        state: State::Proven,
        narrows: &[
            "the radio-slot instance of release_slot, with the same globals and helpers as the primary instance",
        ],
    },
    Row {
        class: "AudioTask",
        method: "release_secondary_audio_task",
        state: State::Proven,
        narrows: &[
            "the secondary-slot instance of release_slot, with the same globals and helpers as the primary instance",
        ],
    },];

/// Counts of (proven, lifted-but-unproven, missing) rows.
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
