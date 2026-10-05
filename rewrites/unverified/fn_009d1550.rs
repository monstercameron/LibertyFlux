// original: 0x009D1550 cross_join_notify (proposed)
//
/// Runs a virtual check over an outer/inner element cross join.
///
/// For each of the `countA` (`[arg+0x18]`, signed) outer elements at
/// `[arg+0x14]` and each of the `countB` (`[arg+0x24]`, signed) inner
/// elements at `[arg+0x20]`, calls the virtual slot at `+0x2c` of the object
/// at `[this+4]` with `(obj, [arg], [arg+4], outer, [arg+0x10], inner, 0)`.
/// A negative (signed) answer appends the `(outer, inner)` pair through the
/// pair helper (callee 2 on `arg + 0x44`; the pair travels through a frame
/// slot observed via the call snapshot). The virtual call is made by loading
/// the slot through the fabricated object exactly like the original.
/// Returns nothing meaningful. Thiscall, one pointer argument.
lf_checker_rt::export!(thiscall, rw_009D1550(this: u32, arg: u32) -> u32 {
    unsafe {
        const OBJ: u32 = 0x04;
        const VSLOT: u32 = 0x2c;
        const APPEND: u32 = 2;
        const TARGET: u32 = 0x44;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        type VFn = extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
        let count_a = rd(arg.wrapping_add(0x18));
        if (count_a as i32) <= 0 {
            return 0;
        }
        let count_b = rd(arg.wrapping_add(0x24));
        let arr_a = rd(arg.wrapping_add(0x14));
        let arr_b = rd(arg.wrapping_add(0x20));
        let a0 = rd(arg);
        let a4 = rd(arg.wrapping_add(4));
        let a10 = rd(arg.wrapping_add(0x10));
        let obj = rd(this.wrapping_add(OBJ));
        let vtab = rd(obj);
        let slot = rd(vtab.wrapping_add(VSLOT));
        let f: VFn = core::mem::transmute(slot as usize);
        let mut i = 0u32;
        while (i as i32) < (count_a as i32) {
            let outer = rd(arr_a.wrapping_add(i.wrapping_mul(4)));
            let mut j = 0u32;
            while (j as i32) < (count_b as i32) {
                let inner = rd(arr_b.wrapping_add(j.wrapping_mul(4)));
                let ans = f(vtab, obj, a0, a4, outer, a10, inner, 0);
                if (ans as i32) < 0 {
                    let mut pair = [outer, inner];
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        APPEND, u32, arg.wrapping_add(TARGET),
                        (&mut pair as *mut u32) as u32);
                }
                j += 1;
            }
            i += 1;
        }
        0
    }
});
