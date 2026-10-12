//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit method of the composer classes this module
//! covers: the combining filter, the optimised data block and the composer's
//! own methods. `Proven` means the method is restated on its type and the
//! `lf-composerdiff` test crate ran it against its verified rewrite on the
//! same generated inputs, comparing results, every call in order and every
//! written word, with a deliberately wrong copy caught alongside. `Missing`
//! means the method is verified but not lifted here, with the reason. The
//! counts below come from this table.

/// Lift state of one verified method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on its type and proven against its rewrite.
    Proven,
    /// Not lifted, for the stated reason.
    Missing,
}

/// One verified method's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// Class, e.g. `"CombiningFilter"`.
    pub class: &'static str,
    /// Slot or method name in the 32-bit form, e.g. `"vf3"`.
    pub method: &'static str,
    /// The lifted name, or "not lifted".
    pub lifted_as: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the method is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Narrowing shared by the composer forwarders: the callee and the sync are
/// scripted to write none of the composer's words, the original returns zero
/// while the lift returns nothing, and a null composer is outside the domain.
const FORWARD_SCRIPTED: &str =
    "the callee and the sync are scripted to write none of the composer's words the lift holds; the lift reads them through a shared reference";
const FORWARD_ZERO: &str =
    "the original returns zero; the lift returns nothing, and the proof checks the rewrite's zero";
const NULL_COMPOSER: &str = "a null composer faults in the original and is outside the proof";

/// Every verified method of the composer classes, in class order.
pub const ROWS: &[Row] = &[
    Row {
        class: "CombiningFilter",
        method: "vf0",
        lifted_as: "CombiningFilter::initialise",
        state: State::Proven,
        narrows: &[
            "the vtable word the original installs is not held by the lift; the harness checks it is the class's vtable",
            "the child count word is the length of the child list; the lift has no separate count",
            "the owner is found by the caller (thread-local block read); the lift takes it as an argument",
            "registration without an owner is outside the domain (panics); a null filter faults in the original, so the proof never passes one",
        ],
    },
    Row {
        class: "CombiningFilter",
        method: "vf3",
        lifted_as: "CombiningFilter::poll",
        state: State::Proven,
        narrows: &[
            "child counts are proven from zero to eight: the original's child list is inline from offset 12 to 44, so a larger count reads the count and mode words as children",
            "a child count of zero or less is an empty child list; the original's signed test is not represented separately",
            "mode bits past the thirty-second child are coded as the original's shift does, but the inline list never reaches them",
            "children are polled through a boxed trait object; the child's own memory is outside the lift",
        ],
    },
    Row {
        class: "CombiningFilter",
        method: "vf4",
        lifted_as: "CombiningFilter::fold",
        state: State::Proven,
        narrows: &[
            "the base-class callee is given no access to the filter: the lift reads the mode word after the call and the proof scripts the callee to write nothing",
            "the rewrite stores nothing; the lift returns the fold and changes nothing",
        ],
    },
    Row {
        class: "ComposerOptimizedData",
        method: "vf0",
        lifted_as: "ComposerOptimizedData::scalar_deleting_destructor",
        state: State::Proven,
        narrows: &[
            "the thread's manager is found by the caller (thread-local block read); the lift takes it as an argument",
            "the free of the block is a call to the manager; the lift has no allocation to release, so the owner drops the block after the call",
            "a block to be freed without a manager is outside the domain (panics)",
            "the proof scripts both callees to write no block memory; their own effects are the trait implementers' responsibility",
        ],
    },
    Row {
        class: "ComposerOptimizedData",
        method: "vf2",
        lifted_as: "ComposerOptimizedData::reset_tables",
        state: State::Proven,
        narrows: &[
            "a value entry's two words are modelled; the other bytes of each entry's stride are not held and the harness checks they are unchanged",
            "a pointer-table entry holds a block-relative offset; the harness adds the block's address on the original's side",
            "the header callee is scripted to write no block memory; its effects are the trait implementers' responsibility",
            "the returned offset is the block-relative form of the address the original returns",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf20",
        lifted_as: "ComposerOptimized::advance_sequence",
        state: State::Proven,
        narrows: &[
            "the refill slot is scripted to write none of the three words the lift holds; the step is applied after the call, in the original's order",
            "the two stack words the original takes and never reads are not passed",
            NULL_COMPOSER,
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf25",
        lifted_as: "ComposerOptimized::advance_and_read_word",
        state: State::Proven,
        narrows: &[
            "the refill slot is scripted to write none of the words the lift holds; the word at offset 0x18 is read after the call, as the original reads it",
            "the two stack words are not passed",
            NULL_COMPOSER,
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf19",
        lifted_as: "ComposerOptimized::forward_two_words_pose",
        state: State::Proven,
        narrows: &[
            "the callee (proposed name apply_pose_direct, unverified) is a trait method; its effects are the implementer's",
            FORWARD_SCRIPTED,
            FORWARD_ZERO,
            NULL_COMPOSER,
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf24",
        lifted_as: "ComposerOptimized::forward_two_words_tracks",
        state: State::Proven,
        narrows: &[
            "the callee (proposed name apply_static_tracks_to_pose, unverified) is a trait method; its effects are the implementer's",
            FORWARD_SCRIPTED,
            FORWARD_ZERO,
            NULL_COMPOSER,
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf21",
        lifted_as: "ComposerOptimized::forward_one_word_copy",
        state: State::Proven,
        narrows: &[
            "the callee (proposed name copy_dof_pair, unverified) is a trait method; its effects are the implementer's",
            "the second argument is the constant zero, passed as the original passes it",
            FORWARD_SCRIPTED,
            FORWARD_ZERO,
            NULL_COMPOSER,
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf26",
        lifted_as: "ComposerOptimized::forward_one_word_setup",
        state: State::Proven,
        narrows: &[
            "the callee (proposed name setup_identity_frame, unverified) is a trait method; its effects are the implementer's",
            "the second argument is the constant zero, passed as the original passes it",
            FORWARD_SCRIPTED,
            FORWARD_ZERO,
            NULL_COMPOSER,
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf2",
        lifted_as: "ComposerOptimized::drain_and_forward",
        state: State::Proven,
        narrows: &[
            "the drain callee (proposed name apply_static_tracks, unverified) and the inner object's forward are trait methods; the lift reads no memory through the inner handle",
            "the free-node cache is an opaque handle set to none; its pool is not modelled",
            "the sync is called unconditionally, as the original calls it, while the forwarders skip it in immediate mode",
            "a composer without an inner object faults in the original and panics here",
            "the drain callee is scripted to write none of the words the lift holds",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf23",
        lifted_as: "ComposerOptimized::peek_or_pop",
        state: State::Proven,
        narrows: &[
            "the inner stack is passed by the caller: the lift does not reach the inner object through its handle; the proof checks the caller's copy matches the object's words",
            "a top index past the table panics; the original would read beyond the table",
            "the refill slot is scripted to write none of the words the lift holds",
            "only the low byte of the mode word decides the pop; the proof covers both values of that byte",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf1",
        lifted_as: "not lifted",
        state: State::Missing,
        narrows: &[
            "stores and returns a free-node pointer that is the inner block's address plus a constant: address arithmetic on a block whose layout is not modelled yet",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf6",
        lifted_as: "ComposerOptimized::gate_drain_forward",
        state: State::Proven,
        narrows: &[
            "the out-word is passed as a Rust location plus an identity the gate callee receives; the proof maps the identity to the location",
            "the gate's answer is compared by its low byte only, as the original does; the lift returns a boolean",
            "the drain and the forward are trait methods; the drain callee and the inner forward are scripted to write none of the words the lift holds",
            "a composer without an inner object, or an accepting gate without an out-word, is outside the domain (panics)",
            "the free-node cache is an opaque handle set to none; its pool is not modelled",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf7",
        lifted_as: "not lifted",
        state: State::Missing,
        narrows: &[
            "records a 16-byte vector operation into the pending pool: needs the node pool and the record layout for this opcode",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf8",
        lifted_as: "not lifted",
        state: State::Missing,
        narrows: &[
            "records a two-float scalar operation into the pending pool: needs the node pool and the record layout for this opcode",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf9",
        lifted_as: "not lifted",
        state: State::Missing,
        narrows: &[
            "records an object-plus-floats operation into the pending pool: needs the node pool and the record layout for this opcode",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf10",
        lifted_as: "not lifted",
        state: State::Missing,
        narrows: &[
            "records a conditionally routed operation into the pending pool: needs the node pool and the record layout for this opcode",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf11",
        lifted_as: "not lifted",
        state: State::Missing,
        narrows: &[
            "records a clamped-float operation into the pending pool: needs the node pool and the record layout for this opcode",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf12",
        lifted_as: "not lifted",
        state: State::Missing,
        narrows: &[
            "records a table-block copy operation into the pending pool: needs the node pool and the record layout for this opcode",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf13",
        lifted_as: "not lifted",
        state: State::Missing,
        narrows: &[
            "records a table-block copy operation (second opcode) into the pending pool: needs the node pool and the record layout",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf14",
        lifted_as: "not lifted",
        state: State::Missing,
        narrows: &[
            "records a float-plus-word operation into the pending pool: needs the node pool and the record layout for this opcode",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf15",
        lifted_as: "not lifted",
        state: State::Missing,
        narrows: &[
            "records a conditional node link into the pending pool: needs the node pool and the record layout for this opcode",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf16",
        lifted_as: "not lifted",
        state: State::Missing,
        narrows: &[
            "records a single-word operation into the pending pool: needs the node pool and the record layout for this opcode",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf17",
        lifted_as: "not lifted",
        state: State::Missing,
        narrows: &[
            "records a pointer-pair operation into the pending pool: needs the node pool and the record layout for this opcode",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf18",
        lifted_as: "not lifted",
        state: State::Missing,
        narrows: &[
            "records a marker operation into the pending pool: needs the node pool and the record layout for this opcode",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf22",
        lifted_as: "ComposerOptimized::dispatch_table_entry",
        state: State::Proven,
        narrows: &[
            "the handler callee (numbered callee 2, meaning not established) is a trait method; its effects are the implementer's",
            "the stack is passed by the caller, as for the stack read; the lift does not reach the inner object through its handle",
            "the table entry is an opaque handle; a zero entry is passed as a null identity, as the original passes zero",
            "the top index past the table panics; the original would read beyond the table",
            "the sequence step is one, not the two of the sequence methods; the proof checks the counter's value",
            NULL_COMPOSER,
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf30",
        lifted_as: "ComposerOptimized::drain_pending",
        state: State::Proven,
        narrows: &[
            "the link words of the pool nodes are held by the caller in a map keyed by node identity; the lift reads and writes no other node word",
            "the sync (slot 0x74) and the handler slot are trait methods, scripted to write none of the composer's words; the handler's effects are the implementer's",
            "the original returns zero; the lift returns nothing, and the proof checks the rewrite's zero",
            "a node the walk reaches that the caller did not record panics: the original would read memory the caller did not describe",
            "the walk's free-node cache is the lift's own field; a null composer faults in the original and is outside the proof",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf31",
        lifted_as: "ComposerOptimized::gate_pending",
        state: State::Proven,
        narrows: &[
            "the gate's word argument is passed through as a plain word; the gate's own effects on the nodes are the implementer's",
            "the link words are held by the caller in a map keyed by node identity, as for the drain",
            "the sync (slot 0x74) is scripted to write none of the composer's words; the gate is scripted to write none of the words the lift holds",
            "a node the pass reaches that the caller did not record panics",
            "the original's return is the low byte of its flag; the lift returns a boolean and the proof checks the one and zero",
        ],
    },
    Row {
        class: "ComposerOptimized",
        method: "vf33",
        lifted_as: "ComposerOptimized::handle_pending_node",
        state: State::Proven,
        narrows: &[
            "the node is an opaque handle handed to the handler slot; the lift does not read the node",
            "the done word is passed as a Rust location; the proof maps the address the original writes to that location",
            "the handler slot is scripted to write none of the composer's words; the return is always one, as the original returns",
        ],
    },
];

/// Verified methods proven against their rewrites.
#[must_use]
pub fn proven_count() -> usize {
    ROWS.iter().filter(|r| r.state == State::Proven).count()
}

/// Verified methods not lifted here.
#[must_use]
pub fn missing_count() -> usize {
    ROWS.iter().filter(|r| r.state == State::Missing).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_proven_row_names_what_it_narrows() {
        for row in ROWS.iter().filter(|r| r.state == State::Proven) {
            assert!(!row.narrows.is_empty(), "{} {} narrows nothing listed", row.class, row.method);
        }
    }

    #[test]
    fn the_rows_cover_the_verified_methods_of_the_three_classes() {
        assert_eq!(ROWS.len(), 31);
        assert_eq!(proven_count(), 18);
        assert_eq!(missing_count(), 13);
    }
}
