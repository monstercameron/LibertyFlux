// original: 0x00662960 rage::snJoinSessionTask::vf7

/// Run one join-session tick: an optional detach, a probe pair, a mode
/// dispatch, then one of two finish sequences.
///
/// `this` is the task object; `a0` selects the finish, `a1` is handed to the
/// tail callee. When `+DETACH` is non-zero, callee 1 (thiscall, one word:
/// `this+0x56c`, `this = inner[+0x24]`) runs first.
///
/// Main finish (`a0 == 1`): callee 7 (thiscall: 1, `a1`) runs, `+INNER` is
/// cleared, and a ten-word descriptor is built for callee 13 (thiscall, one
/// word: a frame pointer): the relocated tag, a self pointer, two zeros, the
/// `+F38` dword, the `+F3C` word, the `+F40` dword, the `+F44` word, the
/// `+368` address or zero, and `+568`. Only the tag word is compared; the
/// self pointer sits in the middle of the struct so the words after it
/// cannot be snapshotted contiguously. Then callee 9 (thiscall, four words:
/// `this+0x98`, `+158`, the `+15c` address or zero, `+35c`) runs, callee 10
/// (thiscall, one word: `this+D8`) answers a heap object or null, and a
/// non-null answer gains bits 0 and 1 at its `+0x80` byte. The
/// security-cookie check ends it.
///
/// Side finish (`a0 != 1`): a `+STATE` above 1 (signed) runs callee 2 twice
/// (thiscall, two words: `+8B8`, `+8BC`); when both answers are non-zero,
/// callee 3 (thiscall, one word: the second answer) runs. Bit 0 of the flag
/// byte at `+FLAG` runs callee 4 (thiscall, no stack words) and callee 1
/// again, then clears the bit. A `+MODE2` that is neither below 1 (signed)
/// nor exactly 2 makes the virtual call `obj[0][+0x1c](obj, a0, 0)` through
/// the object at `+OBJ`. A `+GATE` of 1 dispatches on `+STATE`: 3 calls the
/// imported interlocked compare-exchange (stdcall, three words) and stores
/// -1 past the gate on an answer of 1; 4 or 7 runs callee 5 (thiscall, one
/// word); anything else runs callee 6 (thiscall, one word: -1). A `+364` of
/// 6 resets `+364` and `+568` to zero. Callee 7 runs as (thiscall: `a0`,
/// `a1`), `+INNER` is cleared, callee 8 (thiscall, four words:
/// `this+0x108`, `+364`, the `+368` address or zero, `+568`) takes a frame
/// pointer to an untouched zero word, callee 13 takes another, and the
/// cookie check ends it.
///
/// Original: thiscall, two stack words, `/GS` cookie. No return value is set.
lf_checker_rt::export!(thiscall, rw_00662960(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const INNER: u32 = 0x60;
        const FLAG: u32 = 0x32f8;
        const STATE: u32 = 0x90;
        const TAG: u32 = 0x00fe3410;
        const COOKIE: u32 = 0x01057fb4;

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
        unsafe fn rd16z(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn cookie() {
            unsafe {
                let ck = lf_checker_rt::global::<u32>(COOKIE).read_unaligned();
                lf_checker_rt::callee_thiscall!(14, u32, ck);
            }
        }

        if field(this, 0x574) != 0 {
            let ix = field(this, INNER);
            lf_checker_rt::callee_thiscall!(1, u32, field(ix, 0x24), this.wrapping_add(0x56c));
        }
        if a0 == 1 {
            lf_checker_rt::callee_thiscall!(7, u32, this, 1, a1);
            let inner = field(this, INNER);
            let d = field(this, 0x568);
            wr32(this.wrapping_add(INNER), 0);
            let c = if d != 0 { this.wrapping_add(0x368) } else { 0 };
            let mut s = [
                lf_checker_rt::relocated(TAG),
                0,
                0,
                0,
                field(this, 0xf38),
                rd16z(this.wrapping_add(0xf3c)),
                field(this, 0xf40),
                rd16z(this.wrapping_add(0xf44)),
                c,
                d,
            ];
            s[1] = s.as_ptr() as u32;
            lf_checker_rt::callee_thiscall!(13, u32, inner, s.as_ptr() as u32);
            let e0 = field(this, 0x35c);
            let e = if e0 != 0 { this.wrapping_add(0x15c) } else { 0 };
            lf_checker_rt::callee_thiscall!(
                9, u32, inner, this.wrapping_add(0x98), field(this, 0x158), e, e0
            );
            let r: u32 =
                lf_checker_rt::callee_thiscall!(10, u32, inner, this.wrapping_add(0xd8));
            if r != 0 {
                wr8(r.wrapping_add(0x80), rd8(r.wrapping_add(0x80)) | 3);
            }
            cookie();
            return 0;
        }
        if (field(this, STATE) as i32) > 1 {
            let inner = field(this, INNER);
            let r: u32 = lf_checker_rt::callee_thiscall!(
                2, u32, inner, field(this, 0x8b8), field(this, 0x8bc)
            );
            if r != 0 {
                let r2: u32 = lf_checker_rt::callee_thiscall!(
                    2, u32, inner, field(this, 0x8b8), field(this, 0x8bc)
                );
                if r2 != 0 {
                    lf_checker_rt::callee_thiscall!(3, u32, inner, r2);
                }
            }
        }
        let inner = field(this, INNER);
        if rd8(inner.wrapping_add(FLAG)) & 1 != 0 {
            lf_checker_rt::callee_thiscall!(4, u32, inner.wrapping_add(0xc6c));
            lf_checker_rt::callee_thiscall!(1, u32, field(inner, 0x24), inner.wrapping_add(0x28));
            wr8(inner.wrapping_add(FLAG), rd8(inner.wrapping_add(FLAG)) & !1);
        }
        let h = field(this, 0x7a4) as i32;
        if !(h < 1 || h == 2) {
            let obj = this.wrapping_add(0x798);
            let slot: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(obj).wrapping_add(0x1c)) as usize);
            slot(obj, a0, 0);
        }
        let gate = this.wrapping_add(0x78c);
        if field(this, 0x78c) == 1 {
            match field(this, STATE) {
                3 => {
                    let cas: extern "stdcall" fn(u32, u32, u32) -> u32 = core::mem::transmute(
                        lf_checker_rt::global::<u32>(0x00e731ec).read_unaligned() as usize,
                    );
                    if cas(gate, 2, 1) == 1 {
                        wr32(gate.wrapping_add(4), 0xffff_ffff);
                    }
                }
                4 | 7 => {
                    lf_checker_rt::callee_thiscall!(
                        5, u32, field(this, INNER).wrapping_add(0x48), gate
                    );
                }
                _ => {
                    lf_checker_rt::callee_thiscall!(6, u32, gate, 0xffff_ffff);
                }
            }
        }
        if field(this, 0x364) == 6 {
            wr32(this.wrapping_add(0x364), 0);
            wr32(this.wrapping_add(0x568), 0);
        }
        lf_checker_rt::callee_thiscall!(7, u32, this, a0, a1);
        let inner2 = field(this, INNER);
        let c = field(this, 0x568);
        wr32(this.wrapping_add(INNER), 0);
        let e = if c != 0 { this.wrapping_add(0x368) } else { 0 };
        let z = 0u32;
        lf_checker_rt::callee_thiscall!(
            8, u32, &z as *const u32 as u32,
            this.wrapping_add(0x108), field(this, 0x364), e, c
        );
        let z2 = 0u32;
        lf_checker_rt::callee_thiscall!(13, u32, inner2, &z2 as *const u32 as u32);
        cookie();
        0
    }
});
