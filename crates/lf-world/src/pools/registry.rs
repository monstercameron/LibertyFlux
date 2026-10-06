//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit routine of the covered pool structures:
//! the slot descriptors, the pool vectors, the lookup tables and scans,
//! the fixed tables, the small resets, the handle-indexed pages, the row
//! tables, the wide vectors, the small allocators, and the routines of
//! the remaining structures (mutex-guarded pools, helper-call cluster,
//! sweeps, singles) as missing rows with reasons.
//! `Proven` means the routine is restated on its pool type
//! and the differential test crate ran it against its verified rewrite on
//! the same generated inputs, comparing results and every effect, with a
//! deliberately wrong lift caught alongside. Counts below come from this
//! table.

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
        method: "PoolVec::init_constructed",
        state: State::Proven,
        narrows: &[
            "the 12-byte header travels as its count; the spare word and body pointer stay at the boundary (pinned per case)",
            "elements travel as byte slots built through a trait; call order and slot addresses compare per call",
            "the empty answer narrows from the block address to offset 0",
        ],
    },
    Row {
        func: "pool_init_stride_60",
        method: "WideVec::init",
        state: State::Proven,
        narrows: &[
            "the 16-byte header travels as its count; the spare word and base pointer stay at the boundary (pinned per case)",
            "the element stamp travels as an opaque handle",
            "element bytes past the stamp and status word are allocator garbage in the original and backing as it came in the lift (uncompared)",
            "strides below 12 with a non-empty pool panic; the original writes past its buffer",
            "proof layouts avoid address wrap (checked per case)",
        ],
    },
    Row {
        func: "pool_init_stride_70",
        method: "WideVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_init_stride_60 over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_init_stride_80_a",
        method: "WideVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_init_stride_60 over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_init_stride_160",
        method: "WideVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_init_stride_60 over its own stride and stamp; same narrowings",
        ],
    },
    Row {
        func: "pool_init_stride_70_wide",
        method: "WideVec::init_extra",
        state: State::Proven,
        narrows: &[
            "same header, stamp and backing narrowings as pool_init_stride_60",
            "the bumped end narrows to the extra end offset (the block itself, as offset 0, when empty)",
            "strides below 0x68 with a non-empty pool panic; the original writes past its buffer",
        ],
    },
    Row {
        func: "pool_init_stride_3d0_constructed",
        method: "WideVec::init_constructed",
        state: State::Proven,
        narrows: &[
            "the 16-byte header travels as its count; the spare word and base pointer stay at the boundary (pinned per case)",
            "elements travel as byte slots built through a trait; call order and slot addresses compare per call",
            "the empty answer narrows from the block address to offset 0",
        ],
    },
    Row {
        func: "pool_init_stride_80_b",
        method: "WideVec::init",
        state: State::Proven,
        narrows: &[
            "same routine as pool_init_stride_60 over its own stride and stamp; same narrowings",
        ],
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
        method: "BumpPool::next",
        state: State::Proven,
        narrows: &[
            "the element base travels as a call argument; the proof compares the slot and the bumped count",
        ],
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
        narrows: &[
            "its whole content is one forwarded call with two constant addresses; not lifted by rule",
        ],
    },
    Row {
        func: "pool_obj_create_3e80",
        method: "PublishedObj::create",
        state: State::Proven,
        narrows: &[
            "the global slot becomes the return value; the vtable stays an opaque handle",
            "the instance parameters travel as arguments",
        ],
    },
    Row {
        func: "pool_obj_create_13880",
        method: "PublishedObj::create",
        state: State::Proven,
        narrows: &[
            "same routine as pool_obj_create_3e80 over its own parameters and slot; same narrowings",
        ],
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
    // Fixed tables: the 20-byte entries and the 44-byte revocation table.
    Row {
        func: "pool_entry_table_init",
        method: "EntryTable::init",
        state: State::Proven,
        narrows: &[
            "the table base stays at the boundary (pinned per case)",
            "the end-pointer answer narrows to the biased end offset (end + 8, as the 32-bit form computes it from 8 past the base)",
            "proof layouts avoid address wrap (checked per case)",
        ],
    },
    Row {
        func: "pool_entry_find_mark",
        method: "EntryTable::find_mark",
        state: State::Proven,
        narrows: &[
            "the hit answer narrows from index * 5 to the index (the proof reconstructs it); the 0x13FB miss to None",
            "proof layouts avoid address wrap (checked per case)",
        ],
    },
    Row {
        func: "pool_table_reset",
        method: "RevocTable::reset",
        state: State::Proven,
        narrows: &[
            "the slot-reset object stays opaque behind a trait; its ignored answer is uncompared",
            "the final cursor narrows to the entry-byte length (the proof adds the flag bias)",
            "cursor ranges crossing 2^31 are out of domain (the proof pins small addresses)",
        ],
    },
    Row {
        func: "pool_table_clear",
        method: "RevocTable::clear",
        state: State::Proven,
        narrows: &[
            "the final cursor narrows to the entry-byte length (the proof adds the flag bias)",
            "cursor ranges crossing 2^31 are out of domain (the proof pins small addresses)",
        ],
    },
    Row {
        func: "pool_entry_revoke",
        method: "RevocTable::revoke",
        state: State::Proven,
        narrows: &[
            "the live count travels as the first header word",
            "the final cursor narrows to the entry-byte length (the proof adds the flag bias)",
            "cursor ranges crossing 2^31 are out of domain (the proof pins small addresses)",
        ],
    },
    // Small resets and slot pairs.
    Row {
        func: "pool_handle_state_reset",
        method: "HandleState::reset",
        state: State::Proven,
        narrows: &["the echoed object address carries no meaning"],
    },
    Row {
        func: "pool_slot_reset",
        method: "SmallSlot::reset",
        state: State::Proven,
        narrows: &["the original leaves the return register untouched; the rewrite answers 0"],
    },
    Row {
        func: "pool_row_pairs_zero",
        method: "RowPairs::zero_all",
        state: State::Proven,
        narrows: &["the end-cursor answer narrows to the end offset"],
    },
    Row {
        func: "pool_set_pair_a08",
        method: "SlotPair::set_pair",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        func: "pool_set_pair_a10",
        method: "SlotPair::set_pair",
        state: State::Proven,
        narrows: &["same routine as pool_set_pair_a08 over the high slots; same narrowings (none)"],
    },
    // Handle-indexed pages.
    Row {
        func: "pool_slot_datum_field",
        method: "HandlePool::datum_field",
        state: State::Proven,
        narrows: &["out-of-range handles panic; the original reads the wrapped table address"],
    },
    Row {
        func: "pool_slot_flag_bit15",
        method: "HandlePool::flag_bit",
        state: State::Proven,
        narrows: &[
            "out-of-range handles panic; the original reads the wrapped table address",
            "the 1/0 answer narrows to bool",
        ],
    },
    Row {
        func: "pool_slot_flag_bit10",
        method: "HandlePool::flag_bit",
        state: State::Proven,
        narrows: &["same routine as pool_slot_flag_bit15 over bit 10; same narrowings"],
    },
    // Row tables.
    Row {
        func: "pool_row_count_guarded",
        method: "RowTable::count_guarded",
        state: State::Proven,
        narrows: &[
            "the enable byte travels as bool",
            "rows past the table panic; the original reads on",
            "the row count is the table length",
        ],
    },
    Row {
        func: "pool_row_cell",
        method: "RowTable::cell",
        state: State::Proven,
        narrows: &["indexes past their table panic; the original reads on"],
    },
    Row {
        func: "pool_cell_iterator",
        method: "RowCursor::step",
        state: State::Proven,
        narrows: &[
            "the 1/0 answer narrows to bool",
            "negative start rows panic; the original reads before the table",
            "the row count is the table length",
        ],
    },
    // Mutex-guarded pools.
    Row {
        func: "pool_lookup_build_a",
        method: "-",
        state: State::Missing,
        narrows: &[
            "hash lookup with build-on-miss under the pool mutex; needs a lock trait in lf-platform; not reached",
        ],
    },
    Row {
        func: "pool_lookup_build_b",
        method: "-",
        state: State::Missing,
        narrows: &["mutex-guarded lookup instance; not reached"],
    },
    Row {
        func: "pool_lookup_build_c",
        method: "-",
        state: State::Missing,
        narrows: &["mutex-guarded lookup instance; not reached"],
    },
    Row {
        func: "pool_lookup_build_indirect",
        method: "-",
        state: State::Missing,
        narrows: &["mutex-guarded lookup instance; not reached"],
    },
    Row {
        func: "pool_init32",
        method: "-",
        state: State::Missing,
        narrows: &["32-slot init through the TLS allocator; needs an allocator trait; not reached"],
    },
    Row {
        func: "pool_guarded_build",
        method: "-",
        state: State::Missing,
        narrows: &[
            "guarded build through the pool helper with a fixed callback; needs the lock trait; not reached",
        ],
    },
    Row {
        func: "pool_guarded_build_passthrough",
        method: "-",
        state: State::Missing,
        narrows: &["guarded build passing all words; needs the lock trait; not reached"],
    },
    Row {
        func: "pool_slot_rebind",
        method: "-",
        state: State::Missing,
        narrows: &["binary-search rebind under two mutexes; needs the lock trait; not reached"],
    },
    // Row/cell/stage cluster.
    Row {
        func: "pool_foreach_accumulate",
        method: "-",
        state: State::Missing,
        narrows: &["chain walk accumulating through head/value callees; not reached"],
    },
    Row {
        func: "pool_dispatch_by_index",
        method: "-",
        state: State::Missing,
        narrows: &["index-to-slot switch tail-calling the slot setter; not reached"],
    },
    Row {
        func: "pool_select_slot_a",
        method: "-",
        state: State::Missing,
        narrows: &["discriminator-selected slot offset forwarded to the slot setter; not reached"],
    },
    Row {
        func: "pool_select_slot_b",
        method: "-",
        state: State::Missing,
        narrows: &["discriminator-selected slot offset forwarded to the slot setter; not reached"],
    },
    Row {
        func: "pool_try_stage_transitions",
        method: "-",
        state: State::Missing,
        narrows: &["one stage-machine step through probe/stage callees; not reached"],
    },
    Row {
        func: "pool_stage_at_offset48",
        method: "-",
        state: State::Missing,
        narrows: &[
            "one forwarded stage call on the +0x48 sub-object with a low-byte answer; not reached",
        ],
    },
    Row {
        func: "pool_init_capacity_10000",
        method: "-",
        state: State::Missing,
        narrows: &["its whole content is one forwarded call with a constant; not lifted by rule"],
    },
    Row {
        func: "pool_bind_or_invalidate",
        method: "-",
        state: State::Missing,
        narrows: &["bind to the shared manager with handle/cookie checks; not reached"],
    },
    Row {
        func: "pool_replace_head",
        method: "-",
        state: State::Missing,
        narrows: &["head replace through unlink/link callees on opaque nodes; not reached"],
    },
    Row {
        func: "pool_init_empty_list",
        method: "-",
        state: State::Missing,
        narrows: &["empty-list init under the shared lock; needs the lock design; not reached"],
    },
    Row {
        func: "pool_find_slot",
        method: "-",
        state: State::Missing,
        narrows: &[
            "slot-array scan tracking change points, reading before the array; needs the record-array design; not reached",
        ],
    },
    Row {
        func: "pool_grow_array",
        method: "-",
        state: State::Missing,
        narrows: &["append with first-use allocation through the allocator callee; not reached"],
    },
    Row {
        func: "pool_activate_slot",
        method: "-",
        state: State::Missing,
        narrows: &["slot activation through two stage callees plus flagging; not reached"],
    },
    Row {
        func: "pool_alloc_array",
        method: "-",
        state: State::Missing,
        narrows: &[
            "array alloc with per-entry header zeroing; needs an allocator trait; not reached",
        ],
    },
    Row {
        func: "pool_free_array",
        method: "-",
        state: State::Missing,
        narrows: &["notify-then-free per entry plus the array; not reached"],
    },
    Row {
        func: "pool_forward_lookup",
        method: "-",
        state: State::Missing,
        narrows: &["kind-indexed handler tail-call; not reached"],
    },
    // Helper-call cluster.
    Row {
        func: "pool_guarded_lookup",
        method: "-",
        state: State::Missing,
        narrows: &["session-helper lookup with a kind-selected entry; not reached"],
    },
    Row {
        func: "pool_release_list",
        method: "-",
        state: State::Missing,
        narrows: &["list release through per-node function-table slots; not reached"],
    },
    Row {
        func: "pool_chain_search",
        method: "-",
        state: State::Missing,
        narrows: &["24-byte node chain search through helper callees; not reached"],
    },
    Row {
        func: "pool_record_clone",
        method: "-",
        state: State::Missing,
        narrows: &["record clone with per-field overrides and string copies; not reached"],
    },
    Row {
        func: "pool_record_append",
        method: "-",
        state: State::Missing,
        narrows: &["fully-specified record append with string copies; not reached"],
    },
    Row {
        func: "pool_row_hook_dispatch",
        method: "-",
        state: State::Missing,
        narrows: &["hook dispatch through a row's hook pointer; not reached"],
    },
    Row {
        func: "pool_notify_nodes",
        method: "-",
        state: State::Missing,
        narrows: &["node notification from the iterator and row lists; not reached"],
    },
    Row {
        func: "pool_double_query_compare",
        method: "-",
        state: State::Missing,
        narrows: &["two-field query compared against caller slots (narrow proof); not reached"],
    },
    Row {
        func: "pool_datum_word_via_helper",
        method: "-",
        state: State::Missing,
        narrows: &["datum read through the pool helper on the looked-up page; not reached"],
    },
    Row {
        func: "pool_datum_bounded_via_helper",
        method: "-",
        state: State::Missing,
        narrows: &["bounded datum read through the pool helper; not reached"],
    },
    Row {
        func: "pool_word_table_via_helper",
        method: "-",
        state: State::Missing,
        narrows: &["signed word-table read through the pool helper; not reached"],
    },
    Row {
        func: "pool_indexed_cell_bounded",
        method: "-",
        state: State::Missing,
        narrows: &["index-helper cell read with a row-limit check; not reached"],
    },
    Row {
        func: "pool_two_field_fetch",
        method: "-",
        state: State::Missing,
        narrows: &["chained-helper two-field fetch into two outputs; not reached"],
    },
    Row {
        func: "pool_indexed_cell",
        method: "-",
        state: State::Missing,
        narrows: &["index-helper cell read with no bounds check; not reached"],
    },
    Row {
        func: "pool_datum_field_via_helper",
        method: "-",
        state: State::Missing,
        narrows: &["helper-call field read at result +0x40; not reached"],
    },
    Row {
        func: "pool_guarded_enable",
        method: "-",
        state: State::Missing,
        narrows: &["enable plus readiness-helper refresh; not reached"],
    },
    Row {
        func: "pool_flag_bit6_via_helpers",
        method: "-",
        state: State::Missing,
        narrows: &[
            "flag-bit test behind the chained helpers (one routine, three instances); not reached",
        ],
    },
    Row {
        func: "pool_flag_bit4_via_helpers",
        method: "-",
        state: State::Missing,
        narrows: &["chained-helper flag-bit instance; not reached"],
    },
    Row {
        func: "pool_flag_bit1_via_helpers",
        method: "-",
        state: State::Missing,
        narrows: &["chained-helper flag-bit instance; not reached"],
    },
    Row {
        func: "pool_triple_query_cell_test",
        method: "-",
        state: State::Missing,
        narrows: &["triple-query cell-emptiness test; not reached"],
    },
    // Sweeps and gates.
    Row {
        func: "pool_sweep_matching_into_list",
        method: "-",
        state: State::Missing,
        narrows: &["double sweep prepending matching cells, gated on the enable byte; not reached"],
    },
    Row {
        func: "pool_slot_search",
        method: "-",
        state: State::Missing,
        narrows: &["helper-index slot search with a direct-index path; not reached"],
    },
    Row {
        func: "pool_latched_gate",
        method: "-",
        state: State::Missing,
        narrows: &["latched three-stage gate with threshold/flag select; not reached"],
    },
    Row {
        func: "pool_matching_cells_collect",
        method: "-",
        state: State::Missing,
        narrows: &["row/column matching-cell collect through cell helpers; not reached"],
    },
    Row {
        func: "pool_readiness_gate",
        method: "-",
        state: State::Missing,
        narrows: &["readiness report after ensuring awake; not reached"],
    },
    // Singles.
    Row {
        func: "pool_view_triplet_copy",
        method: "-",
        state: State::Missing,
        narrows: &[
            "copies through a singleton view's triplet pointer; needs the view design; not reached",
        ],
    },
    Row {
        func: "pool_object_set_tracked",
        method: "-",
        state: State::Missing,
        narrows: &[
            "flag writes plus tracker enable/attach callees; needs a tracker-trait design; not reached",
        ],
    },
    Row {
        func: "pool_fit_check",
        method: "-",
        state: State::Missing,
        narrows: &["mode/flag/state-gated fit check with a size-call comparison; not reached"],
    },
    Row {
        func: "pool_array_create",
        method: "-",
        state: State::Missing,
        narrows: &[
            "record-array alloc with per-record header init; needs an allocator trait; not reached",
        ],
    },
    Row {
        func: "pool_array_ctor",
        method: "-",
        state: State::Missing,
        narrows: &[
            "vtable-stamping constructor over head/tail/50 elements; needs a constructor-trait design; not reached",
        ],
    },
    Row {
        func: "pool_distant_spawn",
        method: "-",
        state: State::Missing,
        narrows: &[
            "top-down distant-row scan with a spawn round through seven callees; not reached",
        ],
    },
    Row {
        func: "pool_pick_best",
        method: "-",
        state: State::Missing,
        narrows: &["veto-chain best-pick over row tables with four callees; not reached"],
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
