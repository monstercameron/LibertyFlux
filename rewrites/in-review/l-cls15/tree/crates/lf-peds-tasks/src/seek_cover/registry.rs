//! What is proven for each lifted method, and what each proof narrows.
//!
//! "Proven" means the differential test crate runs the method against its
//! verified 32-bit rewrite on generated inputs and compares the return value,
//! every collaborator call in order with its arguments, a copy of the object
//! at each call, and the bytes the method writes. A wrong version of each
//! proven method is run through the same comparison and is caught. Nothing
//! here is verified by the checker itself.

/// Proof state of one method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// Lifted, and compared with its verified rewrite by the differential
    /// test crate.
    Proven,
    /// Verified by the checker, not lifted yet.
    Missing,
    /// Not verified by the checker: not lifted.
    Unverified,
}

/// One method of the family.
#[derive(Clone, Copy, Debug)]
pub struct Row {
    /// The class the method belongs to.
    pub class: &'static str,
    /// The method's name in the original's inventory.
    pub method: &'static str,
    /// The Rust method that restates it, or `-` when not lifted.
    pub rust: &'static str,
    /// Proof state.
    pub state: State,
    /// What the proof does not cover, or why the method is not lifted.
    pub narrows: &'static [&'static str],
}

/// The rows of the seek-cover family, in inventory order.
pub const ROWS: &[Row] = &[
    Row {
        class: "CTaskComplexSeekCover",
        method: "CTaskComplexSeekCover",
        rust: "SeekCoverTask::construct_full",
        state: State::Proven,
        narrows: &[
            "the vtable word is written by the world (set_stamp); the proof compares it as the same bound value on both sides",
            "the two embedded sub-objects and the base task's words are the world's: their initialisers run as calls and their bytes are not compared, because the stubs write nothing into them",
            "the position vector and the second vector are passed by reference; a null vector pointer faults in the original and is not drawn",
        ],
    },
    Row {
        class: "CTaskComplexSeekCover",
        method: "CTaskComplexSeekCover_2",
        rust: "SeekCoverTask::construct_cover_row",
        state: State::Proven,
        narrows: &[
            "the cover table is bound to test memory of 4096 rows; indices past that row count are not drawn (the original reads another address)",
            "the query writes four words through its out pointer; the proof scripts those words and compares them, so a different out-pointer use is seen only through the stored slot words",
            "the proof does not check the row address against zero: the original's read of the row faults before any such check, and the port reads the row through the world",
        ],
    },
    Row {
        class: "CTaskComplexSeekCover",
        method: "CTaskComplexSeekCover_3",
        rust: "SeekCoverTask::construct_short",
        state: State::Proven,
        narrows: &[
            "as construct_full: the vtable word, the sub-object bytes and the base task's words are the world's",
        ],
    },
    Row {
        class: "CTaskComplexSeekCover",
        method: "~CTaskComplexSeekCover",
        rust: "SeekCoverTask::destruct",
        state: State::Proven,
        narrows: &[
            "the base destructor and the two member releases are calls the world makes; their effects on the sub-objects and the base words are the world's and are not compared",
            "the owned-word release receives the word's value and the address of the word; the proof names both, so a release of the wrong word is seen",
        ],
    },
    Row {
        class: "CTaskComplexSeekCover",
        method: "vf1",
        rust: "SeekCoverTask::route_to_sub_task",
        state: State::Proven,
        narrows: &[
            "routes three to seven are not run: the original faults reading the subtask's latch byte at address zero plus its offset; the port returns none before the call, so no proof covers them",
            "a missing worker or a missing subtask makes the original fault in the same read; the port returns none. Neither is drawn by the proof",
            "the pool is read through the world (fetch_worker); the proof names the pool cell and checks the worker and subtask identities",
        ],
    },
    Row {
        class: "CTaskComplexSeekCover",
        method: "vf21",
        rust: "SeekCoverTask::allocate_node_for_arg",
        state: State::Proven,
        narrows: &[
            "the node's stamp and fields are written by the world (store_small_node); the proof compares the node image, not the path that wrote it",
            "the object is not read by this method; it takes no receiver",
        ],
    },
    Row {
        class: "CTaskComplexSeekCoverShooting",
        method: "vf0",
        rust: "ShootingTask::scalar_delete",
        state: State::Proven,
        narrows: &[
            "the pool object is named in the call log and not otherwise checked",
        ],
    },
    Row {
        class: "CTaskComplexSeekCoverShooting",
        method: "vf1",
        rust: "-",
        state: State::Unverified,
        narrows: &["no verified rewrite in the family's list"],
    },
    Row {
        class: "CTaskComplexSeekCoverShooting",
        method: "get_task_type",
        rust: "-",
        state: State::Unverified,
        narrows: &["no verified rewrite in the family's list"],
    },
    Row {
        class: "CTaskComplexSeekCoverShooting",
        method: "create_next_sub_task",
        rust: "-",
        state: State::Unverified,
        narrows: &["no verified rewrite in the family's list"],
    },
    Row {
        class: "CTaskComplexSeekCoverShooting",
        method: "vf19",
        rust: "ShootingTask::pursuit_action",
        state: State::Proven,
        narrows: &[
            "the solver's scratch buffer is an out-argument of the call: the port does not carry it, and the proof names it as scratch without comparing its contents (the rewrite never reads it)",
            "the virtual slot at offset 0x54 is a dispatch through the object's own vtable: the proof plants that vtable and compares the call by name and argument; the port's dispatch is a trait method",
            "the target's flag byte is read through the world; the proof checks the flag bit by exercising both paths",
        ],
    },
    Row {
        class: "CTaskComplexSeekCoverShooting",
        method: "vf20",
        rust: "-",
        state: State::Missing,
        narrows: &[
            "verified, 2011 bytes: about thirty-seven callees (numbered and virtual), float gates on two constant tables and a global, and a cover-target state machine; not lifted in this lane",
        ],
    },
    Row {
        class: "CTaskComplexSeekCoverShooting",
        method: "vf21",
        rust: "ShootingTask::shot_node",
        state: State::Proven,
        narrows: &[
            "the scale factor is read through the world; the factor is the only float operand that can be not-a-number, so the operand order cannot change its payload",
            "the direction vector is passed by reference and named in the call log",
        ],
    },
    Row {
        class: "CTaskComplexSeekCoverShooting",
        method: "vf5",
        rust: "ShootingTask::initialise_against_subject",
        state: State::Proven,
        narrows: &[
            "a null subject faults in the original (it reads the subject's flag byte at address zero plus its offset); the port returns zero before the call, so no proof covers it",
            "the argument object is non-null in the proof; the port takes it as a handle",
            "the subject's virtual slot at offset 0x14 is planted by the test and compared by name and arguments",
        ],
    },
    Row {
        class: "CTaskComplexSeekCoverShooting",
        method: "vf22",
        rust: "ShootingTask::has_full_chain",
        state: State::Proven,
        narrows: &[
            "a null link in the chain faults in the original when read; the port returns zero. Not drawn by the proof",
            "the type-tag calls are virtual slots at offset 0x0C, planted by the test; the chain links are fixed by the test and the port reads them through the world",
        ],
    },
];
