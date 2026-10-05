// original: 0x009D1730 childlist_teardown_b (proposed)
//
/// Tears down a child-pointer list and resets it.
///
/// Visits the `count` (`[this+0x5c]`, signed; none when not positive)
/// pointers at `[this+0x58]`, running the child teardown (callee 1) and the
/// deleter (callee 2) on each non-null one, then resets the list vector
/// (callee 3 with `0` on `this + 0x58`). All callee returns are ignored.
/// Returns nothing meaningful (`eax` is untouched incoming-register
/// passthrough). Thiscall, no arguments.
lf_checker_rt::export!(thiscall, rw_009D1730(this: u32) -> u32 {
    unsafe {
        const ARR: u32 = 0x58;
        const COUNT: u32 = 0x5c;
        const TEARDOWN: u32 = 1;
        const DELETE: u32 = 2;
        const RESET: u32 = 3;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        let count = rd(this.wrapping_add(COUNT));
        if (count as i32) > 0 {
            let arr = rd(this.wrapping_add(ARR));
            let mut i = 0u32;
            while (i as i32) < (count as i32) {
                let e = rd(arr.wrapping_add(i.wrapping_mul(4)));
                if e != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(TEARDOWN, u32, e);
                    let _: u32 = lf_checker_rt::callee_cdecl!(DELETE, u32, e);
                }
                i += 1;
            }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            RESET, u32, this.wrapping_add(ARR), 0);
        0
    }
});
