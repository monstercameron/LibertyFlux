// original: 0x00DF1C50 UIMontageText::vf94

/// Select one of eight child entries by a probed key, then re-probe the
/// selected pair and record whether it still matches.
///
/// `this` points to the montage-text object. Eight child pointers live at
/// `PROBES` (pairs at `0x1e8`, `0x1f8`, `0x200`, `0x208`; the `0x1f0` word is
/// the match flag, not a child). Each child answers a 32-bit key through the
/// virtual slot at `+GET_SLOT`, called thiscall-style with no stack arguments.
///
/// The eight children are probed in order against `want`. The first child
/// whose key equals `want` selects its pair: when `want` differs from the
/// last-seen key at `+SEEN_OFF`, a finder routine (callee 4, fixed object and
/// one per-pair constant argument) runs and `+SEEN_OFF` is updated. The pair's
/// two children become the pick (`+PICK_OFF`) and the re-probe target; the
/// ready byte at `+FLAG_OFF` is set to 1, the second child is probed again,
/// and the match byte at `+MATCH_OFF` records whether the re-probed key still
/// equals `want`.
///
/// When no child matches, a reset routine (callee 2, object read from a
/// global, single argument 1) runs and the fallback child at `+FALLBACK` is
/// probed. A mismatch there returns the probed key unchanged. A match probes
/// the fallback once more and passes that key to the virtual slot at
/// `+SET_SLOT`, returning its result.
///
/// Return value: in the pair path the re-probed key with its low byte replaced
/// by the match flag (the original sets only `al`); otherwise the last probed
/// key or the `+SET_SLOT` result.
///
/// Original: 0x00DF1C50 (thiscall, ecx = this, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_00df1c50(this: u32, want: u32) -> u32 {
    unsafe {
        const GET_SLOT: u32 = 0x4c;
        const SET_SLOT: u32 = 0x178;
        const FALLBACK: u32 = 0x1e4;
        const FLAG_OFF: u32 = 0x1e0;
        const MATCH_OFF: u32 = 0x1f0;
        const PICK_OFF: u32 = 0x214;
        const SEEN_OFF: u32 = 0x21c;
        const PROBES: [u32; 8] = [0x1e8, 0x1ec, 0x1f8, 0x1fc, 0x200, 0x204, 0x208, 0x20c];
        const PAIR_CONST: [u32; 4] = [0x00F0114C, 0x00F01168, 0x00F01190, 0x00F011B0];
        const FINDER_OBJ: u32 = 0x01176888;
        const RESET_GLOBAL: u32 = 0x018B6C8C;
        const FINDER_CALLEE: u32 = 4;
        const RESET_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        /// Call virtual slot `slot` on `obj` thiscall-style through the
        /// object's own table, exactly like the original; both sides land on
        /// the same planted checker stub.
        #[inline(always)]
        unsafe fn vcall(obj: u32, slot: u32) -> u32 {
            unsafe {
                let target = rd32(rd32(obj) + slot);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(target as usize);
                f(obj)
            }
        }

        let mut hit = 8u32;
        for (i, off) in PROBES.iter().enumerate() {
            if vcall(rd32(this + off), GET_SLOT) == want {
                hit = i as u32;
                break;
            }
        }
        if hit < 8 {
            let pair = hit / 2;
            if want != rd32(this + SEEN_OFF) {
                lf_checker_rt::callee_thiscall!(
                    FINDER_CALLEE,
                    u32,
                    lf_checker_rt::relocated(FINDER_OBJ),
                    lf_checker_rt::relocated(PAIR_CONST[pair as usize])
                );
                wr32(this + SEEN_OFF, want);
            }
            let base = PROBES[(pair * 2) as usize];
            let first = rd32(this + base);
            let second = rd32(this + base + 4);
            wr32(this + PICK_OFF, first);
            wr8(this + FLAG_OFF, 1);
            let got = vcall(second, GET_SLOT);
            let flag = u32::from(want == got);
            wr8(this + MATCH_OFF, flag as u8);
            (got & 0xFFFF_FF00) | flag
        } else {
            let reset_obj = rd32(lf_checker_rt::relocated(RESET_GLOBAL));
            lf_checker_rt::callee_thiscall!(RESET_CALLEE, u32, reset_obj, 1);
            let obj = rd32(this + FALLBACK);
            let got = vcall(obj, GET_SLOT);
            if got != want {
                return got;
            }
            let again = vcall(obj, GET_SLOT);
            let target = rd32(rd32(obj) + SET_SLOT);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            f(obj, again)
        }
    }
});
