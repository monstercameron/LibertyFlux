//! What is proven and what each proof narrows: one row per verified
//! `audio_slot_*` routine of the covered structures.
//!
//! Proven rows name the lifted method and every narrowing of its proof;
//! Missing rows name the reason. The counts are pinned by a host test.

/// Proof state of one verified routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowState {
    /// Lifted and proven against the verified rewrite on the 32-bit target.
    Proven,
    /// Not lifted; the reason is in the row.
    Missing,
}

/// One registry row: a verified routine and its proof state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Row {
    /// Short name (the `audio_slot_*` name).
    pub name: &'static str,
    /// Proof state.
    pub state: RowState,
    /// What the lift narrows, or why the routine is missing.
    pub note: &'static str,
}

/// Banked voice slots: five proven of six.
pub const BANKED: &[Row] = &[
    Row {
        name: "audio_slot_lookup_store",
        state: RowState::Proven,
        note: "indexes past the modelled slots panic (the store writes on); stride 0 with nonzero value panics (the divide faults)",
    },
    Row {
        name: "audio_slot_node_lookup",
        state: RowState::Proven,
        note: "null answers narrow to None; the proof rebuilds the node address from the key per case",
    },
    Row {
        name: "audio_slot_retrigger",
        state: RowState::Proven,
        note: "node addresses narrow to keys (rebuilt per case); the setup call's trailing zero is a placeholder dropped at the boundary; the table base travels as an opaque u32",
    },
    Row {
        name: "audio_slot_op_forward",
        state: RowState::Proven,
        note: "node addresses narrow to keys (rebuilt per case)",
    },
    Row {
        name: "audio_slot_probe",
        state: RowState::Proven,
        note: "1/0 answers narrow to bool; node reads cross the trait; addresses rebuilt per case",
    },
    Row {
        name: "audio_slot_release_forward",
        state: RowState::Missing,
        note: "one virtual forward, no behaviour: not lifted",
    },
];

/// The voice list: three proven of four.
pub const VOICE_LIST: &[Row] = &[
    Row {
        name: "audio_slot_alloc (bitset)",
        state: RowState::Proven,
        note: "records, bits and the dword table past the stored data panic; the guard addresses never cross the trait (the lock is opaque)",
    },
    Row {
        name: "audio_slot_alloc (dword table)",
        state: RowState::Proven,
        note: "the table length is fixed at 0x258 words",
    },
    Row {
        name: "audio_slot_sweep",
        state: RowState::Proven,
        note: "voice handles narrow to (bank, slot) keys (rebuilt per case); lock calls carry no addresses",
    },
    Row {
        name: "audio_slot_voice_update",
        state: RowState::Missing,
        note: "deferred: 481 bytes over five callees with flag updates and a stage pair; needs its own lane",
    },
];

/// Small pools: three proven of three.
pub const POOLS: &[Row] = &[
    Row {
        name: "audio_slot_alloc (strided pool)",
        state: RowState::Proven,
        note: "element words travel as opaque u32 (byte-exact); cells past the modelled table panic; the setup call's object address is pinned, not passed",
    },
    Row {
        name: "audio_slot_alloc (triplet table)",
        state: RowState::Proven,
        note: "hit/miss answers narrow to Some(index)/None (arg0 and the end address pinned per case); the signed scan agrees while the table stays on one side of the sign boundary (asserted per case)",
    },
    Row {
        name: "audio_slot_array_release",
        state: RowState::Proven,
        note: "slot addresses narrow to indexes (rebuilt per case)",
    },
];

/// Single-routine neighbours: four proven of six.
pub const MISC: &[Row] = &[
    Row {
        name: "audio_slot_add_and_dispatch",
        state: RowState::Proven,
        note: "the object address is pinned, not passed; only the sum crosses the trait",
    },
    Row {
        name: "audio_slot_init (slot header)",
        state: RowState::Proven,
        note: "constructor: bytes outside +0x00..=0x3c are untouched on both sides",
    },
    Row {
        name: "audio_slot_release_gated",
        state: RowState::Proven,
        note: "the bank lookup is assumed not to retarget the slot word (pinned per case: the release receives the planted slot)",
    },
    Row {
        name: "audio_slot_liveness_gate",
        state: RowState::Proven,
        note: "the mixer object address is pinned, not passed; the view address rebuilds from the object per case",
    },
    Row {
        name: "audio_slot_ptr_from_top",
        state: RowState::Missing,
        note: "an address routine: its whole content is one address from globals; nothing to own",
    },
    Row {
        name: "audio_slot_ptr_from_base",
        state: RowState::Missing,
        note: "an address routine: its whole content is one address from globals; nothing to own",
    },
];

/// Registration thunks and large deferred routines.
pub const REST: &[Row] = &[
    Row {
        name: "audio_slot_bind_* (x19) and audio_slot_bind_x6",
        state: RowState::Missing,
        note: "registration thunks: one init call plus one register call, no behaviour",
    },
    Row {
        name: "audio_slot_init_* (x15 event slots)",
        state: RowState::Missing,
        note: "registration thunks: one setup call plus one register call, no behaviour",
    },
    Row {
        name: "audio_slot_clear",
        state: RowState::Missing,
        note: "unconditional zero-fill of five words, no behaviour",
    },
    Row {
        name: "audio_slot_update",
        state: RowState::Missing,
        note: "deferred: 528 bytes of float filtering over two callees; needs its own lane",
    },
    Row {
        name: "audio_slot_gather_filter",
        state: RowState::Missing,
        note: "deferred: 527 bytes of gather, helper scoring and float filtering; needs its own lane",
    },
];

/// Proven routine count across all sections (thunk runs count per member).
#[must_use]
pub const fn proven_count() -> usize {
    5 + 3 + 3 + 4
}

/// Missing routine count across all sections (thunk runs count per member).
#[must_use]
pub const fn missing_count() -> usize {
    1 + 1 + 2 + (19 + 1 + 15 + 1 + 1 + 1)
}
