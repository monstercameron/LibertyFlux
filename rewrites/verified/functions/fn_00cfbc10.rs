// original: 0x00cfbc10 climb_ladder_reset_state (proposed)

/// Reset a climb-ladder task object to its initial state.
///
/// `this` points to an object of at least 0x1448 bytes. Every field listed
/// below is zeroed except the state sentinel at `+0x1444`, which is set to -1.
/// Two embedded arrays of eleven entries are cleared field by field: entries of
/// 0xb0 bytes starting at `+0xb0` and entries of 0xe0 bytes starting at
/// `+0x900`, six words per entry (the first two and the last four words of
/// each entry's head). The loop counts down from 10 and exits on sign, so it
/// runs eleven times (counter 9 down to -1). The remaining stores clear
/// scattered header, middle
/// and tail words, including two 8-byte zero fills. Nothing is read except
/// `this` itself; the function returns `this`.
///
/// Original: 0x00cfbc10 (thiscall, no stack arguments, returns ecx in eax).
lf_checker_rt::export!(thiscall, rw_00cfbc10(this: u32) -> u32 {
    unsafe {
        const HEAD_A: u32 = 0x20;
        const HEAD_B: u32 = 0x24;
        const HDR0: u32 = 0x40;
        const HDR1: u32 = 0x44;
        const HDR2: u32 = 0x48;
        const HDR3: u32 = 0x4c;
        const ARR_A_BASE: u32 = 0xb0;
        const ARR_A_STRIDE: u32 = 0xb0;
        const ARR_B_BASE: u32 = 0x900;
        const ARR_B_STRIDE: u32 = 0xe0;
        const ARR_ITERS: u32 = 11;
        /// Word offsets within one array entry head, relative to the entry base.
        const ENTRY_WORDS: [u32; 6] = [0x00, 0x04, 0x20, 0x24, 0x28, 0x2c];
        const MID: [u32; 6] = [0x840, 0x844, 0x860, 0x864, 0x868, 0x86c];
        const TAIL_A: [u32; 6] = [0x12a0, 0x12a4, 0x12c0, 0x12c4, 0x12c8, 0x12cc];
        const TAIL_B: [u32; 4] = [0x13a0, 0x13a4, 0x13a8, 0x13ac];
        const TAIL_Q0: u32 = 0x13b0;
        const TAIL_W0: u32 = 0x13b8;
        const TAIL_Q1: u32 = 0x13bc;
        const TAIL_W1: u32 = 0x13c4;
        const SENTINEL: u32 = 0x1444;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(this + HEAD_A, 0);
        wr32(this + HEAD_B, 0);
        wr32(this + HDR0, 0);
        wr32(this + HDR1, 0);
        wr32(this + HDR2, 0);
        wr32(this + HDR3, 0);
        let mut k = 0u32;
        while k < ARR_ITERS {
            let e = this + ARR_A_BASE + k * ARR_A_STRIDE;
            for w in ENTRY_WORDS {
                wr32(e + w, 0);
            }
            k += 1;
        }
        for o in MID {
            wr32(this + o, 0);
        }
        k = 0;
        while k < ARR_ITERS {
            let e = this + ARR_B_BASE + k * ARR_B_STRIDE;
            for w in ENTRY_WORDS {
                wr32(e + w, 0);
            }
            k += 1;
        }
        for o in TAIL_A {
            wr32(this + o, 0);
        }
        for o in TAIL_B {
            wr32(this + o, 0);
        }
        wr32(this + TAIL_Q0, 0);
        wr32(this + TAIL_Q0 + 4, 0);
        wr32(this + TAIL_W0, 0);
        wr32(this + TAIL_Q1, 0);
        wr32(this + TAIL_Q1 + 4, 0);
        wr32(this + TAIL_W1, 0);
        wr32(this + HEAD_A, 0);
        wr32(this + SENTINEL, 0xffff_ffff);
        this
    }
});
