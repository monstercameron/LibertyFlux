// original: 0x009D13E0 childlist_teardown_nested (proposed)
//
/// Tears down a child-pointer list whose children own five vectors each.
///
/// Visits the `count` (`[this+0x13c]`, signed; none when not positive)
/// pointers at `[this+0x138]`. Each non-null child has one vector reset
/// through callee 1 (`0` on `child + 0x44`) and four through callee 2 (`0` on
/// `child + 0x38`, `+0x2c`, `+0x20`, `+0x14` in that order), then is passed
/// to the deleter (callee 3). Afterwards the list vector itself is reset
/// twice (callee 2 with `0` on `this + 0x138`). All callee returns are
/// ignored. Returns nothing meaningful (`eax` is untouched
/// incoming-register passthrough). Thiscall, no arguments.
lf_checker_rt::export!(thiscall, rw_009D13E0(this: u32) -> u32 {
    unsafe {
        const ARR: u32 = 0x138;
        const COUNT: u32 = 0x13c;
        const RESET_A: u32 = 1;
        const RESET_B: u32 = 2;
        const DELETE: u32 = 3;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        let count = rd(this.wrapping_add(COUNT));
        if (count as i32) > 0 {
            let arr = rd(this.wrapping_add(ARR));
            let mut i = 0u32;
            while (i as i32) < (count as i32) {
                let e = rd(arr.wrapping_add(i.wrapping_mul(4)));
                if e != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        RESET_A, u32, e.wrapping_add(0x44), 0);
                    for off in [0x38u32, 0x2c, 0x20, 0x14] {
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            RESET_B, u32, e.wrapping_add(off), 0);
                    }
                    let _: u32 = lf_checker_rt::callee_cdecl!(DELETE, u32, e);
                }
                i += 1;
            }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            RESET_B, u32, this.wrapping_add(ARR), 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            RESET_B, u32, this.wrapping_add(ARR), 0);
        0
    }
});
