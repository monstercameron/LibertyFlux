// original: 0x009D14E0 vec_append_quad (proposed)
//
/// Appends a 16-byte record to a vector, growing it first.
///
/// Asks the capacity helper (callee) to make room for `count + 1` records;
/// a negative (signed) answer aborts with that status. Otherwise copies 16
/// bytes from `src` to `base + count * 16` (the original moves them through
/// vector registers two words at a time; the effect is a plain 16-byte copy),
/// bumps the count at `this + 0x04` and returns 0. Layout: array base at
/// `this + 0x00`, record count at `this + 0x04`. Thiscall, one pointer argument.
lf_checker_rt::export!(thiscall, rw_009D14E0(this: u32, src: u32) -> u32 {
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
        let dst = base.wrapping_add(count.wrapping_mul(16));
        core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, 16);
        (this.wrapping_add(COUNT) as *mut u32).write_unaligned(count.wrapping_add(1));
        0
    }
});
