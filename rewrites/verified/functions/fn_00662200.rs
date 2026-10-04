// original: 0x00662200 rage::snHostSessionTask::vf7

/// Run one host-session tick: mirror two words, rebuild two descriptors,
/// re-derive the session key block, and finish through the shared tail.
///
/// `this` is the task object; `a0` selects the path, `a1` is handed to the
/// tail callee. `inner` is the session object at `+INNER`, whose `+MODE`
/// word gates everything: outside the signed range 2..=3 the function skips
/// straight to the tail.
///
/// Main path (`+MODE` in range, `a0 == 1`): the words at `+MIR_SRC` are
/// copied to `+MIR_DST`; a non-zero `+AUX` runs callee 1 (thiscall, one word:
/// 1), while a zero `+AUX` clears bit 1 of the flag byte at `+FLAG`
/// (the re-check above it always falls through, so its loads are dead and
/// are not repeated here). Callee 2 (thiscall: 1, `a1`) runs then `+INNER`
/// is cleared to zero. Two descriptors are built in succession for callee 3
/// (thiscall, one word: a frame pointer): `[TAG, self, 0, 0]` with tags
/// `TAG1` then `TAG2`, where `self` points at the tag word itself; only the
/// tag is compared, the rest is shape. Bit 5 of `+FLAG` is set between the
/// two. Callees 4 and 5 (thiscall, no stack words) each take a frame pointer
/// to an untouched (zero) word. When `+NINE` is zero, callee 6 (thiscall, one
/// word: the relocated table base) takes another such zero-word pointer.
/// Callee 7 (thiscall, four words) receives four zero words the original
/// stores explicitly below the stack top; its callee-side pop of 16 bytes is
/// what rebalances the stack. Callee 8 (thiscall, four words: a zero-word
/// pointer, `+A0`, the `+4C4` address or zero, `+4C0`) runs, then callee 9
/// (thiscall, one word: a pointer at the constant -1) returns a heap object
/// whose byte at `+0x80` gains bits 0 and 1. The security-cookie check
/// (callee 10, register-preserving) ends this path.
///
/// Side path (`+MODE` in range, `a0 != 1`): `+PROBE` of 1 runs callee 11
/// (thiscall, one word: `this+P`); bit 0 of `+FLAG` set runs callee 12
/// (thiscall, no stack words, `this = inner + 0xc6c`) and callee 13
/// (thiscall, one word: `inner + 0x28`) and then clears bit 0.
///
/// Tail (every path but the main one): callee 2 runs again as
/// (thiscall: `a0`, `a1`), `+INNER` is cleared, and the cookie check ends it.
///
/// Original: thiscall, two stack words, `/GS` cookie. No return value is set.
lf_checker_rt::export!(thiscall, rw_00662200(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const INNER: u32 = 0x60;
        const MODE: u32 = 0x50;
        const FLAG: u32 = 0x32f8;
        const MIR_SRC: u32 = 0xbf0;
        const MIR_DST: u32 = 0xc30;
        const TAG1: u32 = 0xfe3504;
        const TAG2: u32 = 0xfe341c;
        const PROBE: u32 = 0x94;
        const COOKIE: u32 = 0x01057fb4;
        const TABLE_BASE: u32 = 0x019f3230;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn field(this: u32, off: u32) -> u32 {
            unsafe { rd32(this.wrapping_add(off)) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        /// The cookie check: the xor amounts to an unobserved frame-local, so
        /// only the call itself is reproduced, with the cookie as its word.
        #[inline(always)]
        unsafe fn cookie() {
            unsafe {
                let ck = lf_checker_rt::global::<u32>(COOKIE).read_unaligned();
                lf_checker_rt::callee_thiscall!(10, u32, ck);
            }
        }

        let inner = field(this, INNER);
        let mode = field(inner, MODE) as i32;
        let in_range = (2..=3).contains(&mode);
        if in_range && a0 == 1 {
            wr32(inner.wrapping_add(MIR_DST), field(inner, MIR_SRC));
            wr32(inner.wrapping_add(MIR_DST + 4), field(inner, MIR_SRC + 4));
            if field(this, 0xa4) != 0 {
                lf_checker_rt::callee_thiscall!(1, u32, inner, 1);
            } else {
                wr8(inner.wrapping_add(FLAG), rd8(inner.wrapping_add(FLAG)) & !2);
            }
            lf_checker_rt::callee_thiscall!(2, u32, this, 1, a1);
            wr32(this.wrapping_add(INNER), 0);
            // The tags are file VAs with base relocations, so the original
            // stores their relocated values.
            let mut d1 = [lf_checker_rt::relocated(TAG1), 0, 0, 0];
            d1[1] = d1.as_ptr() as u32;
            lf_checker_rt::callee_thiscall!(3, u32, inner, d1.as_ptr() as u32);
            wr8(inner.wrapping_add(FLAG), rd8(inner.wrapping_add(FLAG)) | 0x20);
            let mut d2 = [lf_checker_rt::relocated(TAG2), 0, 0, 0];
            d2[1] = d2.as_ptr() as u32;
            lf_checker_rt::callee_thiscall!(3, u32, inner, d2.as_ptr() as u32);
            let z1 = 0u32;
            lf_checker_rt::callee_thiscall!(4, u32, &z1 as *const u32 as u32);
            let z2 = 0u32;
            lf_checker_rt::callee_thiscall!(5, u32, &z2 as *const u32 as u32);
            if field(this, 0x9c) == 0 {
                let z3 = 0u32;
                lf_checker_rt::callee_thiscall!(
                    6, u32, &z3 as *const u32 as u32,
                    lf_checker_rt::relocated(TABLE_BASE)
                );
            }
            lf_checker_rt::callee_thiscall!(7, u32, inner, 0, 0, 0, 0);
            let c0 = field(this, 0x4c0);
            let e = if c0 != 0 { this.wrapping_add(0x4c4) } else { 0 };
            let z4 = 0u32;
            lf_checker_rt::callee_thiscall!(
                8, u32, inner, &z4 as *const u32 as u32, field(this, 0xa0), e, c0
            );
            let m1 = 0xffff_ffffu32;
            let r9: u32 =
                lf_checker_rt::callee_thiscall!(9, u32, inner, &m1 as *const u32 as u32);
            wr8(r9.wrapping_add(0x80), rd8(r9.wrapping_add(0x80)) | 3);
            cookie();
            return 0;
        }
        if in_range {
            if field(this, PROBE) == 1 {
                lf_checker_rt::callee_thiscall!(
                    11, u32, inner.wrapping_add(0x48),
                    this.wrapping_add(0x94)
                );
            }
            if rd8(inner.wrapping_add(FLAG)) & 1 != 0 {
                lf_checker_rt::callee_thiscall!(12, u32, inner.wrapping_add(0xc6c));
                lf_checker_rt::callee_thiscall!(
                    13, u32, field(inner, 0x24),
                    inner.wrapping_add(0x28)
                );
                wr8(inner.wrapping_add(FLAG), rd8(inner.wrapping_add(FLAG)) & !1);
            }
        }
        lf_checker_rt::callee_thiscall!(2, u32, this, a0, a1);
        wr32(this.wrapping_add(INNER), 0);
        cookie();
        0
    }
});
