// original: 0x009D16D0 filter_append_masked (proposed)
//
/// Appends selected elements of an array to a vector through a filter.
///
/// Scans the `count` (`[this+0x30]`, signed; none when not positive) dwords
/// at `[this+0x2c]`. Each value is offered to the append helper (callee,
/// return ignored) unless skipped: when `[arg1+0x10]` is nonzero the values
/// 2, 4 and 8 are dropped; otherwise, and for every other value, a zero
/// value is kept while a nonzero one is kept only if it shares a bit with
/// the mask at `[arg0+0x1c]`. Kept values travel through a frame slot the
/// callee reads (observed via the call snapshot), so the proof is the exact
/// append sequence. Returns nothing meaningful (`eax` is incoming-register
/// passthrough when no iteration runs). Thiscall, two pointer arguments.
lf_checker_rt::export!(thiscall, rw_009D16D0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const ARRAY: u32 = 0x2c;
        const COUNT: u32 = 0x30;
        const FLAG: u32 = 0x10;
        const MASK: u32 = 0x1c;
        const TARGET: u32 = 0x38;
        const APPEND: u32 = 1;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        let count = rd(this.wrapping_add(COUNT));
        if (count as i32) <= 0 {
            return 0;
        }
        let array = rd(this.wrapping_add(ARRAY));
        let flag = rd(arg1.wrapping_add(FLAG));
        let mask = rd(arg0.wrapping_add(MASK));
        let mut i = 0u32;
        while (i as i32) < (count as i32) {
            let v = rd(array.wrapping_add(i.wrapping_mul(4)));
            let mut keep = false;
            if flag != 0 && (v == 2 || v == 4 || v == 8) {
                keep = false;
            } else if v == 0 {
                keep = true;
            } else if mask & v != 0 {
                keep = true;
            }
            if keep {
                let mut slot = v;
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    APPEND, u32, arg1.wrapping_add(TARGET), (&mut slot as *mut u32) as u32);
            }
            i += 1;
        }
        0
    }
});
