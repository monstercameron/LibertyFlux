// original: 0x00a95750 stream_slot_set_packed_fields (proposed)

/// Pack two argument values into bitfields of a streaming slot object.
///
/// `this` points to the slot. The low byte already stored at `this+0x04` is
/// kept and the first argument is stored above it (`byte0 | (a0 << 8)`), so
/// the word at `+0x04` becomes a 24-bit packed value. The word at `+0x08`
/// keeps its low two bits and takes the second argument shifted up by two
/// (`(old & 3) | (a1 << 2)`).
///
/// Returns the second packed contribution (`a1 << 2`, the value left in eax).
/// Thiscall: object in ecx, two stack words, callee pops 8.
lf_checker_rt::export!(thiscall, rw_00a95750(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const PACKED0: u32 = 0x04;
        const PACKED1: u32 = 0x08;
        const KEEP_MASK: u32 = 3;
        const A0_SHIFT: u32 = 8;
        const A1_SHIFT: u32 = 2;
        let kept = ((this + PACKED0) as *const u8).read() as u32;
        ((this + PACKED0) as *mut u32).write_unaligned(kept | a0.wrapping_shl(A0_SHIFT));
        let old1 = ((this + PACKED1) as *const u32).read_unaligned();
        ((this + PACKED1) as *mut u32)
            .write_unaligned((old1 & KEEP_MASK) | a1.wrapping_shl(A1_SHIFT));
        a1.wrapping_shl(A1_SHIFT)
    }
});
