// original: 0x008872F0 stream_slot_lock (proposed)

/// Lock one row of the stream-slot table through the table lock entry.
///
/// `this` points to the slot-array owner whose table base sits at `+0xe8`.
/// The locked slot is `base + 0x6f1c + row * 0x6f40` (wrapping), passed in
/// `ecx` to the lock entry (callee 1); its answer is the answer of this
/// function.
///
/// Original: 0x008872F0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_008872F0(this: u32, row: u32) -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0xe8;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_BIAS: u32 = 0x6f1c;
        const LOCK: u32 = 1;
        let base = ((this + TABLE_BASE) as *const u32).read_unaligned();
        let slot = base
            .wrapping_add(ROW_BIAS)
            .wrapping_add(row.wrapping_mul(ROW_STRIDE));
        lf_checker_rt::callee_thiscall!(LOCK, u32, slot)
    }
});
