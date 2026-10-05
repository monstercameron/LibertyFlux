// original: 0x00B81A40 thread_wake_commit
/// Commit a pending thread wake-up once its gates agree.
///
/// With no pending wake (`SRC` zero) nothing happens. Otherwise the
/// resolver (callee 1) and two alternative gate probes (callees 2-3)
/// lead to a final confirmation (callee 4) or the global `HOLD` byte;
/// on success the pending fields move to `DST0`/`DST1`, the table at
/// `TAB` gains a 1 at slot `N`, the count advances, a sticky flag is
/// dropped and the pending fields clear.
///
/// Original: 0x00B81A40 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00B81A40(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const SRC: u32 = 0x88;
        const SRC2: u32 = 0x8C;
        const N: u32 = 0x90;
        const DST0: u32 = 0x10;
        const DST1: u32 = 0x14;
        const CNT: u32 = 0x18;
        const TAB: u32 = 0x58;
        const STICKY: u32 = 0x0C;
        const HOLD: u32 = 0x011D923D;
        if rd32(this + SRC) == 0 {
            return 0;
        }
        let hold = || unsafe { (lf_checker_rt::global::<u8>(HOLD)).read() != 0 };
        let confirm = || lf_checker_rt::callee_cdecl!(4, u32,) & 0xFF == 0 || hold();
        let p = lf_checker_rt::callee_cdecl!(1, u32, 0);
        let go = if lf_checker_rt::callee_thiscall!(2, u32, rd32(p + 0x228)) & 0xFF != 0 {
            confirm()
        } else {
            let p2 = lf_checker_rt::callee_cdecl!(1, u32, 0);
            if rd8(p2 + 0x211) != 0 {
                confirm()
            } else {
                let p3 = lf_checker_rt::callee_cdecl!(1, u32, 0);
                if lf_checker_rt::callee_thiscall!(3, u32, rd32(p3 + 0x228)) & 0xFF == 0 {
                    hold()
                } else {
                    confirm()
                }
            }
        };
        if !go {
            return 0;
        }
        wr32(this + DST0, rd32(this + SRC));
        wr32(this + DST1, rd32(this + SRC2));
        let n = rd32(this + N);
        let tab = rd32(this + TAB);
        wr32(this + CNT, n);
        wr32(tab.wrapping_add(n.wrapping_mul(4)), 1);
        wr32(this + CNT, n.wrapping_add(1));
        if rd32(this + STICKY) == 1 {
            wr32(this + STICKY, 0);
        }
        wr32(this + SRC, 0);
        wr32(this + SRC2, 0);
        wr32(this + N, 0);
        0
    }
});
