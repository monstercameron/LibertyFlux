// original: 0x009D1070 vec_reserve_capped (proposed)
//
/// Grows or frees a 24-byte-element vector's storage with capped growth.
///
/// `newcap` must be non-negative (signed) and at most `0x5555555` (unsigned),
/// else `E_INVALIDARG`. A zero `newcap` frees the block (callee 1) and clears
/// base/count/capacity. When a block exists and already covers `newcap`,
/// nothing happens (the fits-check is SIGNED). Otherwise the capacity grows
/// by doubling (from at least 16, clamped to `0x7fffffff` unsigned), then
/// takes the SIGNED maximum of that total and `newcap`, and the block is
/// reallocated (callee 2) to `cap * 24` bytes; a
/// null answer yields `OUTOFMEMORY`, else base and capacity update. A
/// quotient guard (`0xFFFFFFFF / cap >= 0x18`, unsigned) can never fail for
/// in-range `newcap` (the quotient is at least 47) and a zero divisor is
/// impossible (`cap >= 1` there). Layout: base at `this + 0x00`, count at
/// `+0x04`, capacity at `+0x08`. Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_009D1070(this: u32, newcap: u32) -> u32 {
    unsafe {
        const BASE: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const CAP: u32 = 0x08;
        const MAX_CAP: u32 = 0x5555555;
        const MIN_GROW: u32 = 0x10;
        const HARD_CAP: u32 = 0x7fffffff;
        const ELEM: u32 = 24;
        const E_INVALIDARG: u32 = 0x80070057;
        const OOM: u32 = 0x8007000e;
        const FREE: u32 = 1;
        const REALLOC: u32 = 2;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        if (newcap as i32) < 0 || newcap > MAX_CAP {
            return E_INVALIDARG;
        }
        let base = rd(this.wrapping_add(BASE));
        if newcap == 0 {
            if base != 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(FREE, u32, base);
            }
            (this.wrapping_add(BASE) as *mut u32).write_unaligned(0);
            (this.wrapping_add(CAP) as *mut u32).write_unaligned(0);
            (this.wrapping_add(COUNT) as *mut u32).write_unaligned(0);
            return 0;
        }
        let oldcap = rd(this.wrapping_add(CAP));
        if base != 0 && (newcap as i32) <= (oldcap as i32) {
            return 0;
        }
        let grow = if oldcap != 0 { oldcap } else { MIN_GROW };
        let mut total = oldcap.wrapping_add(grow);
        if total > HARD_CAP {
            total = HARD_CAP;
        }
        let cap = if (newcap as i32) <= (total as i32) { total } else { newcap };
        debug_assert!(cap >= 1);
        if 0xffffffffu32 / cap < ELEM {
            return E_INVALIDARG;
        }
        let bytes = cap.wrapping_mul(3).wrapping_mul(8);
        let p: u32 = lf_checker_rt::callee_cdecl!(REALLOC, u32, base, bytes);
        if p == 0 {
            return OOM;
        }
        (this.wrapping_add(BASE) as *mut u32).write_unaligned(p);
        (this.wrapping_add(CAP) as *mut u32).write_unaligned(cap);
        0
    }
});
