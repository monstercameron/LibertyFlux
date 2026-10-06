//! What the input-ui lift proves, per verified routine.
//!
//! One row per listed routine of the lifted shapes; counts are pinned by
//! a host test. Narrowings name what each proof does not cover.

/// Proof state of one verified routine.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProofState {
    /// Lifted and proven against the verified rewrite.
    Proven,
    /// Not lifted; the reason says what is missing.
    Missing,
}

/// One verified routine of a lifted shape.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Row {
    /// Short name (the list's `input_ui_*` name).
    pub name: &'static str,
    /// Proof state.
    pub state: ProofState,
    /// What the proof narrows, or why the routine is missing.
    pub note: &'static str,
}

/// The registry: every listed routine of the lifted shapes.
pub const ROWS: &[Row] = &[
    // The element holders (one owning type, six routines proven).
    Row {
        name: "input_ui_factory_10",
        state: ProofState::Proven,
        note: "vtable narrows to HolderKind::Callback16; sink answer returned as is",
    },
    Row {
        name: "input_ui_factory_0c",
        state: ProofState::Proven,
        note: "vtable narrows to HolderKind::Callback12; sink answer returned as is",
    },
    Row {
        name: "input_ui_create_5arg",
        state: ProofState::Proven,
        note: "constructor body opaque (Construct trait); helper address implied by kind; null alloc panics (the 32-bit form faults)",
    },
    Row {
        name: "input_ui_create_6arg",
        state: ProofState::Proven,
        note: "as input_ui_create_5arg, six-word constructor args",
    },
    Row {
        name: "input_ui_create_tex",
        state: ProofState::Proven,
        note: "as input_ui_create_5arg, three-word constructor args",
    },
    Row {
        name: "input_ui_create_inline",
        state: ProofState::Proven,
        note: "vtable and helper slot narrow to HolderKind::Inline28; null alloc panics (the 32-bit form faults)",
    },
    // Same holder shape, not reached: the element sweeps share the
    // stamp-and-fold pair but need sink/row-helper traits first.
    Row {
        name: "input_ui_build_elements",
        state: ProofState::Missing,
        note: "needs sampler, row-helper and sink traits plus the 1500-entry table layout",
    },
    Row {
        name: "input_ui_cell_build",
        state: ProofState::Missing,
        note: "needs probe, clamp and bind traits plus the cell-table layout",
    },
    Row {
        name: "input_ui_element_from_coords",
        state: ProofState::Missing,
        note: "needs scalar-helper and wide-call traits plus the thread-local gate model",
    },
    Row {
        name: "input_ui_slot_sweep",
        state: ProofState::Missing,
        note: "needs probe and clamp traits plus the slot-table layout",
    },
    Row {
        name: "input_ui_list_refresh",
        state: ProofState::Missing,
        note: "needs emitter and allocator-branch traits plus the row-list layout",
    },
    // The state record (one routine proven).
    Row {
        name: "input_ui_state_copy",
        state: ProofState::Proven,
        note: "destination-address answer narrows to unit; sub-objects behind SubRecord",
    },
    // The bounds accumulator (one routine proven).
    Row {
        name: "input_ui_bounds_accumulate",
        state: ProofState::Proven,
        note: "range ends narrow to an item count (misaligned ranges out of domain); doubled tag pinned on the 32-bit side; NaN results compare as NaN only",
    },
    // The distance gates (two routines proven).
    Row {
        name: "input_ui_float_gate",
        state: ProofState::Proven,
        note: "byte answer narrows to bool; table word, constant words and probe address pinned on the 32-bit side; index confined to a 4-word table",
    },
    Row {
        name: "input_ui_entry_scanner",
        state: ProofState::Proven,
        note: "registry base/stride narrow to slot order (null slots out of domain); entry objects behind traits; vtable slots planted per case",
    },
];

/// Number of proven rows.
#[must_use]
pub const fn proven_count() -> usize {
    let mut n = 0;
    let mut i = 0;
    while i < ROWS.len() {
        if matches!(ROWS[i].state, ProofState::Proven) {
            n += 1;
        }
        i += 1;
    }
    n
}

/// Number of missing rows.
#[must_use]
pub const fn missing_count() -> usize {
    ROWS.len() - proven_count()
}
