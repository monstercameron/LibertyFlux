//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit routine of the slot-descriptor structure:
//! the context cluster, the seven cursor-step instances, the validity
//! check, the release, the three scans and the two script-pool routines.
//! `Proven` means the routine is restated on [`SlotPool`](crate::pools::SlotPool)
//! and the differential test crate ran it against its verified rewrite on
//! the same generated inputs, comparing results and every effect, with a
//! deliberately wrong lift caught alongside. Counts below come from this
//! table.
//!
//! The other pool structures in the lane's list (the pool vectors, the
//! mutex-guarded pools, the row/cell table pools, the inline record
//! arrays) have no rows yet: later lanes add one section each.

/// Lift state of one verified routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on its pool type and proven against its rewrite.
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
    /// Naming-lane name of the verified routine, e.g. `"pool_slot_occupied"`.
    pub func: &'static str,
    /// Lifted method, e.g. `"SlotPool::is_occupied"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the routine is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified routine of the slot-descriptor structure.
pub const ROWS: &[Row] = &[
    Row {
        func: "pool_context_create",
        method: "SlotPool::create_ctx",
        state: State::Proven,
        narrows: &[
            "the context global becomes the return value; the 28-byte block stays opaque (its initialiser is not verified)",
        ],
    },
    Row {
        func: "pool_slot_occupied",
        method: "SlotPool::is_occupied",
        state: State::Proven,
        narrows: &[
            "the slot-address non-null guard is always true over owned entries",
            "indexes past the flag store panic; the original reads past it",
        ],
    },
    Row {
        func: "pool_slot_data_word",
        method: "SlotPool::data_word",
        state: State::Proven,
        narrows: &[
            "dead slots panic; the original reads through null and faults",
            "reads past the entry store panic; the original reads on",
        ],
    },
    Row {
        func: "pool_slot_assign",
        method: "SlotPool::assign",
        state: State::Proven,
        narrows: &[
            "dead slots panic; the original stores through null and faults",
            "the 1/0 answer narrows to bool",
        ],
    },
    Row {
        func: "pool_indexed_store",
        method: "SlotPool::indexed_store",
        state: State::Proven,
        narrows: &[
            "the refresh answer translates from a row base address to a row index",
            "dead slots panic; the original faults through null",
            "cell indexes past the table panic; the original reads past it",
            "proof layouts avoid address wrap (checked per case)",
        ],
    },
    Row {
        func: "pool_slot_valid_check",
        method: "SlotPool::slot_at_offset",
        state: State::Proven,
        narrows: &[
            "the absolute slot address narrows to its offset from the entry base",
            "the residue-carrying 1/0 answer narrows to Option<usize> (the proof pins the low byte and the index)",
            "offsets that wrap the address below the base are out of domain",
            "stride 0 and the MIN/-1 division panic; the original faults",
            "quotients resolving outside the flag store panic; the original reads there",
        ],
    },
    Row {
        func: "pool_iterator_16f7d60",
        method: "SlotPool::cursor_step",
        state: State::Proven,
        narrows: &[
            "the slot-address-or-null answer narrows to Option<usize>; the proof reconstructs the address per case",
            "cursors above the slot count panic; the original reads past the flag store",
            "the null-slot guard never fires on proof layouts (checked per case)",
            "the live count is the flag store length",
        ],
    },
    Row {
        func: "pool_iterator_12bd0e8",
        method: "SlotPool::cursor_step",
        state: State::Proven,
        narrows: &["same routine as pool_iterator_16f7d60 over its own record; same narrowings"],
    },
    Row {
        func: "pool_iterator_166d9ec",
        method: "SlotPool::cursor_step",
        state: State::Proven,
        narrows: &["same routine as pool_iterator_16f7d60 over its own record; same narrowings"],
    },
    Row {
        func: "pool_iterator_18b6f10",
        method: "SlotPool::cursor_step",
        state: State::Proven,
        narrows: &["same routine as pool_iterator_16f7d60 over its own record; same narrowings"],
    },
    Row {
        func: "pool_iterator_1632c60",
        method: "SlotPool::cursor_step",
        state: State::Proven,
        narrows: &["same routine as pool_iterator_16f7d60 over its own record; same narrowings"],
    },
    Row {
        func: "pool_iterator_18b6f1c",
        method: "SlotPool::cursor_step",
        state: State::Proven,
        narrows: &["same routine as pool_iterator_16f7d60 over its own record; same narrowings"],
    },
    Row {
        func: "pool_iterator_12e22a4",
        method: "SlotPool::cursor_step",
        state: State::Proven,
        narrows: &["same routine as pool_iterator_16f7d60 over its own record; same narrowings"],
    },
    Row {
        func: "pool_subsystem_init",
        method: "-",
        state: State::Missing,
        narrows: &[
            "builds the subsystem descriptor from code addresses and allocator probes through four callees; needs a handler-trait design and the callee meanings",
        ],
    },
    Row {
        func: "pool_slot_release",
        method: "SlotPool::release",
        state: State::Proven,
        narrows: &[
            "the null-slot guard is always false over owned entries",
            "the survives answer narrows from a word to its deciding low byte",
            "the constant 0 answer carries no meaning",
        ],
    },
    Row {
        func: "pool_scan_and_report",
        method: "-",
        state: State::Missing,
        narrows: &["top-down scan with a stack-cookie mirror and worker callees; not reached"],
    },
    Row {
        func: "pool_collect_farthest_capped",
        method: "-",
        state: State::Missing,
        narrows: &["capped farthest-first collect over float distances; not reached"],
    },
    Row {
        func: "pool_group_flag_distant_idle",
        method: "-",
        state: State::Missing,
        narrows: &["group scan flagging far idle members; not reached"],
    },
    Row {
        func: "pool_find_matching_slot",
        method: "-",
        state: State::Missing,
        narrows: &[
            "the tracked rewrite does not compile from the repository (undefined pool constants; open review issue), so it cannot be differentially proven until fixed",
        ],
    },
    Row {
        func: "pool_append_converted_string",
        method: "-",
        state: State::Missing,
        narrows: &[
            "same structure, same uncompilable rewrite as pool_find_matching_slot; not reached",
        ],
    },
    // Pool vectors: one initialiser routine, thirteen instances.
    Row {
        func: "pool_vec_init_0x24_791c",
        method: "PoolVec::init",
        state: State::Proven,
        narrows: &[
            "the 12-byte header travels as its count; the spare word and body pointer stay at the boundary (pinned per case)",
            "the element stamp travels as an opaque handle",
            "element bodies past the stamp are allocator garbage in the original and backing as it came in the lift (uncompared)",
            "strides below 4 with more than one slot panic; the original writes past its buffer",
        ],
    },
    Row {
        func: "pool_vec_init_0x10_769c",
        method: "PoolVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_vec_init_0x24_791c over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_vec_init_0x38_771c",
        method: "PoolVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_vec_init_0x24_791c over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_vec_init_0x30_759c",
        method: "PoolVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_vec_init_0x24_791c over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_vec_init_0x38_789c",
        method: "PoolVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_vec_init_0x24_791c over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_vec_init_0x4c_7a1c",
        method: "PoolVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_vec_init_0x24_791c over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_vec_init_0x3c_751c",
        method: "PoolVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_vec_init_0x24_791c over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_vec_init_0x3c_779c",
        method: "PoolVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_vec_init_0x24_791c over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_vec_init_0xa8_7a9c",
        method: "PoolVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_vec_init_0x24_791c over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_vec_init_0x20_799c",
        method: "PoolVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_vec_init_0x24_791c over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_vec_init_0x24_761c",
        method: "PoolVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_vec_init_0x24_791c over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_vec_init_0x1c_7b1c",
        method: "PoolVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_vec_init_0x24_791c over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_vec_init_0x78_781c",
        method: "PoolVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_vec_init_0x24_791c over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_vec_init_ctor_0x6c",
        method: "-",
        state: State::Missing,
        narrows: &[
            "same header and saturated size, but builds each element through a per-element constructor callee; needs a constructor trait",
        ],
    },
    Row {
        func: "pool_init_stride_60",
        method: "-",
        state: State::Missing,
        narrows: &[
            "wide variant: 16-byte header, zeroes element + 8; a second method, not reached",
        ],
    },
    Row {
        func: "pool_init_stride_70",
        method: "-",
        state: State::Missing,
        narrows: &["wide variant instance; not reached"],
    },
    Row {
        func: "pool_init_stride_80_a",
        method: "-",
        state: State::Missing,
        narrows: &["wide variant instance; not reached"],
    },
    Row {
        func: "pool_init_stride_160",
        method: "-",
        state: State::Missing,
        narrows: &["wide variant instance; not reached"],
    },
    Row {
        func: "pool_init_stride_70_wide",
        method: "-",
        state: State::Missing,
        narrows: &["wide variant instance; not reached"],
    },
    Row {
        func: "pool_init_stride_3d0_constructed",
        method: "-",
        state: State::Missing,
        narrows: &["wide variant with constructed elements; not reached"],
    },
    Row {
        func: "pool_init_stride_80_b",
        method: "-",
        state: State::Missing,
        narrows: &["wide variant instance; not reached"],
    },
    Row {
        func: "pool_slot_alloc_0x60",
        method: "-",
        state: State::Missing,
        narrows: &[
            "bump-allocates from global count/base and registers through the slot vtable, hash and registry callees; needs the registry design",
        ],
    },
    Row {
        func: "pool_slot_alloc_0x70_bd90",
        method: "-",
        state: State::Missing,
        narrows: &["typed-pool allocator instance; not reached"],
    },
    Row {
        func: "pool_slot_next_0x20",
        method: "-",
        state: State::Missing,
        narrows: &["bare bump allocator without registration; not reached"],
    },
    Row {
        func: "pool_slot_alloc_0x80_be20",
        method: "-",
        state: State::Missing,
        narrows: &["typed-pool allocator instance; not reached"],
    },
    Row {
        func: "pool_slot_alloc_0x160",
        method: "-",
        state: State::Missing,
        narrows: &["typed-pool allocator instance; not reached"],
    },
    Row {
        func: "pool_slot_alloc_0x70_bf00",
        method: "-",
        state: State::Missing,
        narrows: &["typed-pool allocator instance; not reached"],
    },
    Row {
        func: "pool_slot_alloc_0x3d0",
        method: "-",
        state: State::Missing,
        narrows: &["typed-pool allocator instance; not reached"],
    },
    Row {
        func: "pool_slot_alloc_0x80_bfe0",
        method: "-",
        state: State::Missing,
        narrows: &["typed-pool allocator instance; not reached"],
    },
    Row {
        func: "pool_foreach_callback",
        method: "-",
        state: State::Missing,
        narrows: &["fixed callback over the 0x160-stride pool through one callee; not reached"],
    },
    Row {
        func: "pool_obj_create_3e80",
        method: "-",
        state: State::Missing,
        narrows: &["allocates, initialises and publishes a pool object globally; not reached"],
    },
    Row {
        func: "pool_obj_create_13880",
        method: "-",
        state: State::Missing,
        narrows: &["pool-object creation instance; not reached"],
    },
    Row {
        func: "pool_obj_bind_sync",
        method: "-",
        state: State::Missing,
        narrows: &["binds a fresh object through gates and sync callees; not reached"],
    },
    // Lookup tables: pure searches over pool rows and word arrays.
    Row {
        func: "pool_entry_find",
        method: "TagPools::find",
        state: State::Proven,
        narrows: &[
            "row tags narrow from addresses to keys",
            "null objects and negative starts are out of domain; the original reads on",
            "the row count is the pool length",
        ],
    },
    Row {
        func: "pool_index_search",
        method: "WordTable::search",
        state: State::Proven,
        narrows: &[
            "the needle travels as its value",
            "the limit is the word count",
        ],
    },
    Row {
        func: "pool_pair_init",
        method: "PairPool::init",
        state: State::Proven,
        narrows: &[
            "elements travel as owned values of a generic type; the 0xBD0 stride stays at the boundary (pinned via stub addresses)",
        ],
    },
    Row {
        func: "pool_contains_value",
        method: "WordBlocks::contains",
        state: State::Proven,
        narrows: &["the 1/0 answer narrows to bool"],
    },
    Row {
        func: "pool_clear_match_flags",
        method: "KeyedFlags::clear_matches",
        state: State::Proven,
        narrows: &[
            "entry bodies past the key and flag byte are unmodelled (untouched by both sides)",
        ],
    },
];

/// Number of rows in a state.
#[must_use]
pub const fn count(state: State) -> usize {
    let mut n = 0;
    let mut i = 0;
    while i < ROWS.len() {
        if ROWS[i].state as u8 == state as u8 {
            n += 1;
        }
        i += 1;
    }
    n
}
