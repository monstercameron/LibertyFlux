// original: 0x008E8370 resolve_four_slots (proposed)

/// Resolve four slots through the slot filler, then validate them.
///
/// The slot filler (callee 1, thiscall on `obj`) runs as `(a0, 4, out,
/// MAGIC, a3, 0)`, writing four slot words to the scratch buffer `out`.
/// When the first slot's low word is `EMPTY`, both `*a1` and `*a2` are set
/// to -1 and the filler's answer is returned. Otherwise `*a1` takes the
/// first slot and each later non-empty slot (low word not `EMPTY`) is
/// validated by the checker (callee 2, thiscall on `SLOTS`) as `(first,
/// slot)`; the first slot the checker rejects (low byte 0) is stored to
/// `*a2` and returned, while full acceptance returns the last answer. The
/// trailing security-cookie check (callee 3) is intercepted with register
/// preservation and its cookie argument is not compared.
///
/// The scratch buffer lives in the original's own frame, so its address
/// differs between the sides: the contract skips that argument (no
/// snapshot: the stub writes the slots after the call-time snapshot would
/// run) and observes the slots through the `*a1`/`*a2` writes, the checker
/// arguments and the return value instead.
///
/// Original: 0x008E8370 (thiscall, four stack arguments).
lf_checker_rt::export!(thiscall, rw_008E8370(obj: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        /// Magic word passed to the slot filler.
        const MAGIC: u32 = 0x4974_23FE;
        /// Slot-checker object.
        const SLOTS: u32 = 0x1177A80;
        /// Empty-slot marker (low word).
        const EMPTY: u32 = 0xFFFF;
        /// Slot count requested from the filler.
        const N_SLOTS: u32 = 4;
        /// Slot filler callee id.
        const FILL: u32 = 1;
        /// Slot checker callee id.
        const CHECK: u32 = 2;
        /// Security-cookie check callee id.
        const COOKIE: u32 = 3;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let mut out = [0xFFFF_FFFFu32; 4];
        let out_ptr = &mut out as *mut u32 as u32;
        let ans1: u32 = lf_checker_rt::callee_thiscall!(
            FILL, u32, obj, a0, N_SLOTS, out_ptr, MAGIC, a3, 0
        );
        let first = out[0];
        let mut ans = ans1;
        if (first & 0xFFFF) == EMPTY {
            wr32(a1, 0xFFFF_FFFF);
            wr32(a2, 0xFFFF_FFFF);
        } else {
            wr32(a1, first);
            let mut i: usize = 1;
            while i < out.len() {
                let w = out[i];
                if (w & 0xFFFF) != EMPTY {
                    ans = lf_checker_rt::callee_thiscall!(
                        CHECK,
                        u32,
                        lf_checker_rt::relocated(SLOTS),
                        first,
                        w
                    );
                    if (ans & 0xFF) == 0 {
                        wr32(a2, w);
                        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
                        return w;
                    }
                }
                i += 1;
            }
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        ans
    }
});
