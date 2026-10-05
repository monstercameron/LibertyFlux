// original: 0x00c2cc90 FIRE_START

/// Starts fire audio: builds a voice-request block, opens two voices, mixes.
///
/// `this` points to the fire-sound object (handle slot at `+0x4C`) and `arg`
/// to the requester, whose word at `+0x48` selects an optional voice flavor.
/// Three global gates each abort silently: `ABORT_FLAG` set, `TICK_A`
/// disagreeing with `TICK_B`, or `MODE` holding `MODE_IDLE`.
///
/// Behaviour on the running path:
/// - A refresh call (callee 1), then a request block of 18 words is
///   initialized (callee 2) and customized: byte `FLAG_OFF` OR-ed with
///   `FLAG_BITS`, word 5 set to a scratch pointer, word 8 to the flavor
///   answer (callee 4, only when the flavor word is nonzero), word 9 to
///   the rolling counter `COUNTER`.
/// - The primary voice opens (callee 5): constant tag `TAG0`, out-pointer
///   to the handle slot, the request block, and tail words -1, 0, 0. When
///   the handle comes back nonzero it is attached (callee 6), configured
///   with three zero words (callee 7), given two generated ids (callees 8
///   and 9, the second fed the first's answer), and the ids plus a zero
///   are stored at `+0xA4`/`+0xA8`/`+0xAC`. A null handle keeps id -1.
/// - The secondary voice opens the same way (callee 10, tag `TAG1`), its
///   handle arriving at offset 4 of a two-word frame out-slot; a null
///   handle skips its setup.
/// - The counter advances by `COUNTER_STEP` modulo `COUNTER_MOD` (signed
///   remainder, stored back to the global) and nine constant voice words
///   go to the tuner (callee 11) with `this + 0x0C`.
/// - A final mix call (callee 12) takes the primary id (or -1), the
///   secondary handle, the flavor word, and the request block.
///
/// The request block's word 5 holds a frame address, so snapshots skip it;
/// the remaining 17 words plus the flag byte are compared at each use.
/// Callee answers that arrive in registers are scripted by the checker.
///
/// Original: 0x00c2cc90 (thiscall, one stack word, no return value).
lf_checker_rt::export!(thiscall, rw_00c2cc90(this: u32, arg: u32) -> u32 {
    unsafe {
        const ABORT_FLAG: u32 = 0x11F7060;
        const TICK_A: u32 = 0x12088B4;
        const TICK_B: u32 = 0xF1C040;
        const MODE: u32 = 0x1037720;
        const MODE_IDLE: u32 = 0x12;
        const COUNTER: u32 = 0x16CFD48;
        const COUNTER_STEP: u32 = 0x46;
        const COUNTER_MOD: i32 = 100;
        const HANDLE_SLOT: u32 = 0x4C;
        const FLAVOR: u32 = 0x48;
        const FLAG_OFF: usize = 0x46;
        const FLAG_BITS: u8 = 6;
        const TAG0: u32 = 0xEC6D54;
        const TAG1: u32 = 0xEC6D68;
        const VOICE_ID0: u32 = 0xA4;
        const VOICE_ID1: u32 = 0xA8;
        const VOICE_ID2: u32 = 0xAC;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if rd32(lf_checker_rt::relocated(ABORT_FLAG)) == 1 {
            return 0;
        }
        if rd32(lf_checker_rt::relocated(TICK_A)) != rd32(lf_checker_rt::relocated(TICK_B)) {
            return 0;
        }
        if rd32(lf_checker_rt::relocated(MODE)) == MODE_IDLE {
            return 0;
        }

        lf_checker_rt::callee_thiscall!(1, u32, this);
        let mut req = [0u32; 18];
        lf_checker_rt::callee_thiscall!(2, u32, req.as_mut_ptr() as u32);
        let mut scratch = [0u32; 1];
        lf_checker_rt::callee_thiscall!(3, u32, arg, scratch.as_mut_ptr() as u32);
        *(req.as_mut_ptr() as *mut u8).add(FLAG_OFF) |= FLAG_BITS;
        req[5] = scratch.as_mut_ptr() as u32;
        let seed = rd32(lf_checker_rt::relocated(COUNTER));
        req[9] = seed;
        let flavor = rd32(arg.wrapping_add(FLAVOR));
        if flavor != 0 {
            let ans: u32 = lf_checker_rt::callee_thiscall!(4, u32, flavor);
            req[8] = ans;
        }

        let slot = this.wrapping_add(HANDLE_SLOT);
        lf_checker_rt::callee_thiscall!(
            5, u32, this, TAG0, slot, req.as_mut_ptr() as u32, 0xFFFF_FFFF, 0, 0
        );
        let h0 = rd32(slot);
        let primary = if h0 != 0 {
            lf_checker_rt::callee_thiscall!(6, u32, h0, arg);
            lf_checker_rt::callee_thiscall!(7, u32, h0, 0, 0, 0);
            let id0: u32 = lf_checker_rt::callee_thiscall!(8, u32, h0);
            let id1: u32 = lf_checker_rt::callee_cdecl!(9, u32, id0);
            wr32(h0.wrapping_add(VOICE_ID0), id0);
            wr32(h0.wrapping_add(VOICE_ID1), id1);
            wr32(h0.wrapping_add(VOICE_ID2), 0);
            id0
        } else {
            0xFFFF_FFFF
        };

        let mut out2 = [0u32; 2];
        lf_checker_rt::callee_thiscall!(
            10, u32, this, TAG1, out2.as_mut_ptr() as u32, req.as_mut_ptr() as u32,
            0xFFFF_FFFF, 0, 0
        );
        let h1 = out2[1];
        if h1 != 0 {
            lf_checker_rt::callee_thiscall!(6, u32, h1, arg);
            lf_checker_rt::callee_thiscall!(7, u32, h1, 0, 0, 0);
            let id0: u32 = lf_checker_rt::callee_thiscall!(8, u32, h1);
            let id1: u32 = lf_checker_rt::callee_cdecl!(9, u32, id0);
            wr32(h1.wrapping_add(VOICE_ID0), id0);
            wr32(h1.wrapping_add(VOICE_ID1), id1);
            wr32(h1.wrapping_add(VOICE_ID2), 0);
        }

        let next = (seed.wrapping_add(COUNTER_STEP) as i32 % COUNTER_MOD) as u32;
        wr32(lf_checker_rt::relocated(COUNTER), next);
        lf_checker_rt::callee_thiscall!(
            11, u32, this.wrapping_add(0x0C),
            0x3DCCCCCD, 0x3DCCCCCD, 0, 0x3F000000, 0x3F4CCCCD, 0x3F800000, 0x3E99999A,
            0x3F000000, 0
        );

        let flavor2 = rd32(arg.wrapping_add(FLAVOR));
        lf_checker_rt::callee_cdecl!(12, u32, primary, h1, flavor2, req.as_mut_ptr() as u32);
        0
    }
});
