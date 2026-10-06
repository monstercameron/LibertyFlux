// original: 0x00DF7C60 UIGalleryFileViewer::vf88 (symbols)

/// Gallery refresh with nested nominee-search loops (thiscall, no stack args).
///
/// `this` is the viewer object. The dword tag at `+0x308` selects the entry,
/// the object at `+0x1E8` is the view whose vtable drives the search, byte
/// `+0x314` is a re-entrancy gate (nonzero returns at once) and the dword at
/// `+0x310` is a saturating pass counter (compared SIGNED).
///
/// Behaviour: resolve an entry pointer P, look up a record M through the
/// shared context, and check the guard on M's name string. When the guard
/// fails, skip straight to the second helper call and the counter block
/// (the frame push comes after the guard check, so the stack stays
/// balanced). Otherwise compare two small-id answers; on mismatch save the
/// tag and search: an outer index loop over E.1E0/E.1D4 with an inner
/// nominee loop over Q.1E0/Q.1D4 whose body compares M's and R's name
/// strings byte-wise (unsigned strcmp returning -1/0/+1); on a match store
/// R.4C to the tag. Both loop bounds use UNSIGNED `jb` compares against
/// counts that must stay small (a huge count would not terminate, so
/// signedness there is unobservable by any terminating trial). Finishes
/// with two calls into the gallery helper and the counter block (reset via
/// the pass-reset callee past 6).
lf_checker_lf_checker_rt::export!(thiscall, rb586_fn1(this: u32) -> u32 {
    unsafe {
        const TAG: u32 = 0x308;
        const COUNT: u32 = 0x310;
        const GATE: u32 = 0x314;
        const VIEW: u32 = 0x1e8;
        const CTX: u32 = 0x1981a4c;
        const S_NAME: u32 = 0x23c;
        const S_ID: u32 = 0x4c;
        const S_COUNT: u32 = 0x1d4;
        const S_INDEX: u32 = 0x1e0;
        const C_RESOLVE: u32 = 1;
        const C_LOOKUP: u32 = 2;
        const C_GUARD: u32 = 3;
        const C_HELPER: u32 = 4;
        const C_RESET: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn vcall1(obj: u32, slot: u32, a0: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj, a0)
            }
        }
        /// Unsigned byte-wise strcmp, -1/0/+1 like the original's sbb/or tail.
        unsafe fn strcmp(mut a: u32, mut b: u32) -> i32 {
            unsafe {
                loop {
                    let x = rd8(a);
                    let y = rd8(b);
                    if x != y {
                        return if (x as u32) < (y as u32) { -1 } else { 1 };
                    }
                    if x == 0 {
                        return 0;
                    }
                    a = a.wrapping_add(1);
                    b = b.wrapping_add(1);
                }
            }
        }
        /// The saturating pass counter at +0x310 (signed compare).
        unsafe fn counter(this: u32) {
            unsafe {
                let c = rd32(this + COUNT) as i32;
                if c == 0 {
                    return;
                }
                if c > 6 {
                    lf_checker_rt::callee_thiscall!(C_RESET, u32, this);
                    wr32(this + COUNT, 0);
                } else {
                    wr32(this + COUNT, (c + 1) as u32);
                }
            }
        }

        if rd8(this + GATE) != 0 {
            return 0;
        }
        if rd32(this + TAG) == 0 {
            counter(this);
            return 0;
        }
        let p = lf_checker_rt::callee_thiscall!(C_RESOLVE, u32, this);
        let tag = rd32(this + TAG);
        let m = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, lf_checker_rt::relocated(CTX), tag);
        if p == 0 {
            wr32(this + TAG, 0);
            counter(this);
            return 0;
        }
        let sm = vcall0(m, S_NAME);
        let g: u32 = lf_checker_rt::callee_thiscall!(C_GUARD, u32, this, sm);
        if (g as u8) == 0 {
            lf_checker_rt::callee_thiscall!(C_HELPER, u32, this, p);
            wr32(this + TAG, 0);
            counter(this);
            return 0;
        }
        let e = rd32(this + VIEW);
        let a = vcall0(e, S_ID);
        let b = vcall0(p, S_ID);
        if a == b {
            lf_checker_rt::callee_thiscall!(C_HELPER, u32, this, p);
            wr32(this + TAG, 0);
            counter(this);
            return 0;
        }
        let saved = rd32(this + TAG);
        wr32(this + TAG, 0);
        if vcall0(e, S_COUNT) != 0 {
            let mut outer = 0u32;
            loop {
                let q = vcall1(e, S_INDEX, outer);
                if vcall0(q, S_COUNT) != 0 {
                    let mut inner = 0u32;
                    loop {
                        let r = vcall1(q, S_INDEX, inner);
                        let sp = vcall0(m, S_NAME);
                        let sr = vcall0(r, S_NAME);
                        if strcmp(sp, sr) == 0 {
                            // Match: store R.4C to the tag.
                            wr32(this + TAG, vcall0(r, S_ID));
                            break;
                        }
                        // Mismatch: next nominee while the count allows;
                        // exhaustion skips the R.4C call entirely.
                        inner += 1;
                        if inner < vcall0(q, S_COUNT) {
                            continue;
                        }
                        break;
                    }
                }
                if rd32(this + TAG) != 0 {
                    break;
                }
                outer += 1;
                if outer < vcall0(e, S_COUNT) {
                    continue;
                }
                break;
            }
        }
        lf_checker_rt::callee_thiscall!(C_HELPER, u32, this, e);
        wr32(this + TAG, saved);
        lf_checker_rt::callee_thiscall!(C_HELPER, u32, this, p);
        wr32(this + TAG, 0);
        counter(this);
        0
    }
});
