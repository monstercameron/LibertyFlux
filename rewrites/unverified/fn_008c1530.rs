// original: 0x008C1530 audio_thread_slot_ptr

/// Address one 64-byte audio thread slot from the small object base.
///
/// Same shape as its sibling slot routine with `BASE_BIAS` 0x30 instead of 0x530:
/// adds the bias to the object pointer, loads the thread's audio entry from
/// the thread-local table (`tls_slot(0)`, the word the original reads
/// through FS:[0x2c]) indexed by `TLS_INDEX`, reads the slot base at
/// `ENTRY_BASE` past it, and returns
/// `biased + ((slot_base + index * 4) << 6)`. Pure address arithmetic.
/// Thiscall, object in ECX, index on the stack.
lf_checker_rt::export!(thiscall, rw_008C1530(this: u32, index: u32) -> u32 {
    unsafe {
        const BASE_BIAS: u32 = 0x30;
        const TLS_INDEX: u32 = 0x017A_BA14;
        const ENTRY_BASE: u32 = 0x70;
        const SLOT_SHIFT: u32 = 6;
        let tls = lf_checker_rt::tls_slot(0);
        let biased = this.wrapping_add(BASE_BIAS);
        let tindex = (lf_checker_rt::relocated(TLS_INDEX) as *const u32).read_unaligned();
        let entry = (tls.wrapping_add(tindex.wrapping_mul(4)) as *const u32).read_unaligned();
        let slot_base = (entry.wrapping_add(ENTRY_BASE) as *const u32).read_unaligned();
        slot_base
            .wrapping_add(index.wrapping_mul(4))
            .wrapping_shl(SLOT_SHIFT)
            .wrapping_add(biased)
    }
});
