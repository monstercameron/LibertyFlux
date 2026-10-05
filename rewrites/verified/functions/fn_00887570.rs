// original: 0x00887570 stream_slot_unlock (proposed)

/// Unlock one row of the stream-slot table through the table unlock entry.
///
/// Same slot arithmetic as its lock twin: `base + 0x6f1c + row * 0x6f40`
/// from the table base at `this + 0xe8`, passed in `ecx` to the unlock
/// entry (callee 1); its answer is the answer of this function.
///
/// Original: 0x00887570 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00887570(this: u32, row: u32) -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0xe8;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_BIAS: u32 = 0x6f1c;
        const UNLOCK: u32 = 1;
        let base = ((this + TABLE_BASE) as *const u32).read_unaligned();
        let slot = base
            .wrapping_add(ROW_BIAS)
            .wrapping_add(row.wrapping_mul(ROW_STRIDE));
        lf_checker_rt::callee_thiscall!(UNLOCK, u32, slot)
    }
});
