// original: 0x00c3fef0 timing_block_init (proposed)

/// Initialise a timing parameter block: ten identical 0x30-byte slots
/// followed by a tail.
///
/// Each slot holds two zeroed triples (words at `+0x00` and `+0x10`) and a
/// limit word at `+0x20` set to 45.0. The words at `+0x0c` and `+0x1c` of
/// every slot are deliberately left alone. The tail sets the word at
/// `+0x1e0` to -1, zeroes `+0x1e4`/`+0x1e8`/`+0x1ec` and raises the flag
/// byte at `+0x1f0`.
///
/// Original: 0x00c3fef0 (thiscall, no stack words). Returns `this`.
lf_checker_rt::export!(thiscall, rw_00c3fef0(this: u32) -> u32 {
    unsafe { fef0_core(this, false) }
});

unsafe fn fef0_core(this: u32, skip_first_limit: bool) -> u32 {
    unsafe {
        const N_SLOTS: u32 = 10;
        const SLOT_STRIDE: u32 = 0x30;
        const LIMIT: u32 = 0x4234_0000; // 45.0f
        const TAIL_MARK: u32 = 0x1e0;
        const TAIL_FLAG: u32 = 0x1f0;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        for s in 0..N_SLOTS {
            let g = this + s * SLOT_STRIDE;
            wr32(g, 0);
            wr32(g + 4, 0);
            wr32(g + 8, 0);
            wr32(g + 0x10, 0);
            wr32(g + 0x14, 0);
            wr32(g + 0x18, 0);
            if !(skip_first_limit && s == 0) {
                wr32(g + 0x20, LIMIT);
            }
        }
        wr32(this + TAIL_MARK, 0xffff_ffff);
        wr32(this + TAIL_MARK + 4, 0);
        wr32(this + TAIL_MARK + 8, 0);
        wr32(this + TAIL_MARK + 0xc, 0);
        (this as *mut u8).add(TAIL_FLAG as usize).write(1);
        this
    }
}
