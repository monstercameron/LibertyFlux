// original: 0x00c2e810 audio_emitter_update (proposed, STAGE 1 of 2)
//
// STAGE 1: this rewrite covers the entry head, the three head exits and the
// shared epilogue only. It is verified by a contract whose inputs never fall
// through to the body regions (see `../stage1_plan.md` in this lane's folder
// for the full region map, the call table and the extension recipe). It must
// NOT be recorded as a verified rewrite of the whole function.

/// Audio emitter per-tick update: head flags, liveness chain, epilogue.
///
/// `this` is the emitter (`+0x10` flag word) and `arg` the subject.
/// The head clears flag bit 24, then resolves a live target: `t` starts as
/// `[arg + 0xDC4]` and is discarded (set to 0) when null, when its word at
/// `+0x08` is `0xFFFF`, or when it differs from `[arg + 0x38]`.
/// Three exits share the epilogue: flag bit 18 clear, no live target, or a
/// zero answer from the gate call (callee 1, thiscall/1, `al` answer).
/// The epilogue clears flag bit 9 when `[ebp + 0x10]` equals one less than
/// the global `TICK`.
///
/// Original: 0x00c2e810 (thiscall, three stack words, no return value).
lf_checker_rt::export!(thiscall, rw_00c2e810(this: u32, arg: u32, _f: u32, tick_arg: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x10;
        const CLEAR_HEAD: u32 = 0xFEFF_FFFF;
        const LIVE_BIT: u32 = 0x40000;
        const TARGET: u32 = 0xDC4;
        const TARGET_REF: u32 = 0x38;
        const TARGET_GEN: u32 = 0x08;
        const DEAD_GEN: u32 = 0xFFFF;
        const TICK: u32 = 0x103B694;
        const CLEAR_EPI: u32 = 0xFFFF_FDFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(this.wrapping_add(FLAGS), rd32(this.wrapping_add(FLAGS)) & CLEAR_HEAD);
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        let mut t = rd32(arg.wrapping_add(TARGET));
        if t == 0 || rd16(t.wrapping_add(TARGET_GEN)) as u32 == DEAD_GEN {
            t = 0;
        } else if t != rd32(arg.wrapping_add(TARGET_REF)) {
            t = 0;
        }
        if rd32(this.wrapping_add(FLAGS)) & LIVE_BIT == 0 {
            t = 0;
        }
        if t != 0 {
            let gate: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, arg);
            if gate & 0xFF == 0 {
                t = 0;
            }
        }
        // NOTE: the original falls through to the body regions when a live
        // target survives the gate; stage 1 inputs never do (see plan).
        let _ = t;

        let tick = rd32(lf_checker_rt::relocated(TICK)).wrapping_sub(1);
        if tick_arg == tick {
            wr32(this.wrapping_add(FLAGS), rd32(this.wrapping_add(FLAGS)) & CLEAR_EPI);
        }
        0
    }
});
