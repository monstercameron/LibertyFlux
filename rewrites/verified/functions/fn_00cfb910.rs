// original: 0x00cfb910 climb_ladder_check_state (proposed)

/// Decide whether a climb-ladder task may proceed, and how far.
///
/// `arg0` is the task owner: entry `+0x16c` must be live with kind
/// `+0x28` masked to 0xc0, mode `this+0x18` must avoid 3, 4 and 5, and the
/// height at `arg0+0x170` must reach the global minimum. An indirect-free
/// status call on the owner must answer zero, and a lookup call must return
/// non-null (else a global timer minus `this+0x24` must reach 0x7d0).
///
/// On the lookup path two differences against the position block are checked
/// against the global bound for modes 1 and 2, with per-mode cross checks of
/// the raw height and of the lookup's `+0x18` word. A flag byte must be
/// clear, then five pool slots (0x82-0x86) must all answer zero. Two owner
/// bytes gate the tail: a set-up byte must match the entry's (zero answers
/// zero), and state byte 2 with a live lookup returns the second threshold
/// call's verdict (0x32), with a null lookup the first call's (0x50); other
/// states return the lookup verdict unless both bytes disagree, which
/// re-runs the second threshold call. Returns 1 or 0.
///
/// Original: 0x00cfb910 (thiscall, one stack argument, returns al).
lf_checker_rt::export!(thiscall, rw_00cfb910(this: u32, arg0: u32) -> u32 {
    unsafe {
        const ENTRY_OFF: u32 = 0x16c;
        const KIND_OFF: u32 = 0x28;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_WANT: u32 = 0xc0;
        const MODE_OFF: u32 = 0x18;
        const HEIGHT_OFF: u32 = 0x170;
        const TIMER_SUB: u32 = 0x24;
        const TIMER_MIN: u32 = 0x7d0;
        const LOOKUP_OFF: u32 = 0x224;
        const LOOKUP_BIAS: u32 = 0x44;
        const LOOKUP_ARG: u32 = 0x120;
        const LOOKUP_WORD: u32 = 0x18;
        const POS_OFF: u32 = 0x20;
        const REF_OFF: u32 = 0x38;
        const MY_X_OFF: u32 = 0x38;
        const MY_Y_OFF: u32 = 0x48;
        const FLAG_OFF: u32 = 0x26c;
        const POOL_OFF: u32 = 0x78;
        const SETUP_OFF: u32 = 0x219;
        const STATE_OFF: u32 = 0xa60;
        const STATE_ARMED: u8 = 2;
        const G_MIN_HEIGHT: u32 = 0x00fe_870c;
        const G_BOUND: u32 = 0x00fe_8a24;
        const G_TIMER: u32 = 0x0117_35b4;
        const STATUS: u32 = 1;
        const LOOKUP: u32 = 2;
        const POOLQ: u32 = 3;
        const THRESH: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn rdglobal(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let esi = rd32(arg0 + ENTRY_OFF);
        if esi == 0 {
            return 0;
        }
        if rd32(esi + KIND_OFF) & KIND_MASK != KIND_WANT {
            return 0;
        }
        let mode = rd32(this + MODE_OFF);
        if mode == 3 || mode == 4 || mode == 5 {
            return 0;
        }
        if f32::from_bits(rdglobal(G_MIN_HEIGHT)) > rdf(arg0 + HEIGHT_OFF) {
            return 0;
        }
        let st: u32 = lf_checker_rt::callee_thiscall!(STATUS, u32, arg0);
        if st as u8 != 0 {
            return 0;
        }
        let found: u32 = lf_checker_rt::callee_thiscall!(
            LOOKUP,
            u32,
            rd32(esi + LOOKUP_OFF).wrapping_add(LOOKUP_BIAS),
            LOOKUP_ARG
        );
        let dl = found != 0;
        let mut edx = found;
        if !dl {
            if rdglobal(G_TIMER).wrapping_sub(rd32(this + TIMER_SUB)) < TIMER_MIN {
                return 0;
            }
        } else {
            let base = rd32(arg0 + POS_OFF);
            let bx = rdf(base + REF_OFF);
            let x1 = sub(rdf(this + MY_X_OFF), bx);
            let x0 = sub(bx, rdf(this + MY_Y_OFF));
            // The original tests edx here, but edx is provably non-null on
            // this path (dl was just derived from it and it is unchanged
            // since); mirror the shape anyway.
            if edx == 0 {
                edx = 0;
            } else {
                edx = rd32(edx + LOOKUP_WORD);
            }
            let bound = f32::from_bits(rdglobal(G_BOUND));
            if mode == 1 && x0 > 0.0 && bound > x0 {
                return 0;
            }
            if mode == 2 && x1 > 0.0 && bound > x1 {
                return 0;
            }
            // mode == 3 is dead here (excluded above); kept for fidelity.
            if mode == 3 || mode == 1 {
                if bx > rdf(rd32(esi + POS_OFF) + REF_OFF) {
                    return 0;
                }
            }
            if mode == 2 && rdf(rd32(esi + POS_OFF) + REF_OFF) > bx {
                return 0;
            }
            if mode == 1 && (edx == 1 || edx == 3) {
                return 0;
            }
            if mode == 2 && edx == 2 {
                return 0;
            }
        }
        if rd8(esi + FLAG_OFF) & 1 != 0 {
            return 0;
        }
        let pool = rd32(esi + POOL_OFF);
        let mut ans = 0u32;
        for slot in [0x82u32, 0x83, 0x84, 0x85, 0x86] {
            ans = lf_checker_rt::callee_thiscall!(POOLQ, u32, pool, 9u32, slot);
            if ans != 0 {
                return 0;
            }
        }
        let cl = rd8(arg0 + SETUP_OFF);
        if cl != 0 && rd8(esi + SETUP_OFF) == ans as u8 {
            return 0;
        }
        let state = rd8(arg0 + STATE_OFF);
        if state == STATE_ARMED {
            if rd8(esi + STATE_OFF) != state {
                return 0;
            }
            // The original compares al with itself here (always equal) and
            // re-checks the state byte (dead: just verified, no writes
            // between); both are provably no-ops.
            let r: u32 = lf_checker_rt::callee_cdecl!(THRESH, u32, 0u32, 0x64u32);
            if dl {
                return if (r as i32) >= 0x32 { 1 } else { 0 };
            }
            return if (r as i32) >= 0x50 { 1 } else { 0 };
        }
        if cl == 0 || rd8(esi + SETUP_OFF) == 0 {
            return if dl { 1 } else { 0 };
        }
        let r: u32 = lf_checker_rt::callee_cdecl!(THRESH, u32, 0u32, 0x64u32);
        if (r as i32) >= 0x32 { 1 } else { 0 }
    }
});
