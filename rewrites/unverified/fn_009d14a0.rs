// original: 0x009D14A0 vec_append_pair (proposed)
//
/// Appends an 8-byte record (two dwords) to a vector, growing it first.
///
/// Asks the capacity helper (callee) to make room for `count + 1` records;
/// a negative (signed) answer aborts with that status. Otherwise copies two
/// dwords from `src` to `base + count * 8`, bumps the count at `this + 0x04`
/// and returns 0. Layout: array base at `this + 0x00`, record count at
/// `this + 0x04`. Thiscall, one pointer argument.
lf_checker_rt::export!(thiscall, rw_009D14A0(this: u32, src: u32) -> u32 {
    unsafe {
        const BASE: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const GROW: u32 = 1;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        let count = rd(this.wrapping_add(COUNT));
        let st: u32 = lf_checker_rt::callee_thiscall!(GROW, u32, this, count.wrapping_add(1));
        if (st as i32) < 0 {
            return st;
        }
        let base = rd(this.wrapping_add(BASE));
        let dst = base.wrapping_add(count.wrapping_mul(8));
        (dst as *mut u32).write_unaligned(rd(src));
        (dst.wrapping_add(4) as *mut u32).write_unaligned(rd(src.wrapping_add(4)));
        (this.wrapping_add(COUNT) as *mut u32).write_unaligned(count.wrapping_add(1));
        0
    }
});
