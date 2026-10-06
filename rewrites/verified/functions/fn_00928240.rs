// original: 0x00928240 build_cmd_fold_result_00 (proposed)

/// Allocate a tagged command object, run its handler twice, fold the answers
/// into the tag.
///
/// Allocates an `ALLOC_SIZE`-byte object (callee 1); a null object faults on
/// the first vtable read below, exactly like the original. A live object is
/// tagged like family A (base vtable, counter mixed into `+0x04`), gets `a1`
/// at `+0x08` and `*a2` at `+0x0c` under the final vtable. It then runs the payload constructor (callee 4) on `obj + 0x10` with `a3` and stores `*a4` at `+0x50`. The handler at
/// vtable slot `+0x08` (callee 2, thiscall, intercepted through the planted
/// slot) is then called twice with the object. With `r1`, `r2` the two
/// *signed* answers: `e = r1 % 16`, `s = (16 - e) % 16`, `q = (r2 + s) / 16`
/// (all signed, truncating), and the tag becomes `tag ^= (((q << 14) ^ tag) &
/// 0x1ffc000)`. Returns the folded value.
///
/// Original: 0x00928240 (cdecl, 4 stack words).
lf_checker_rt::export!(cdecl, rw_00928240(a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const BASE_VT: u32 = 0x00E7_E048;
        const FINAL_VT: u32 = 0x00E8_6700;
        const COUNTER: u32 = 0x0103_27A0;
        const ALLOC_SIZE: u32 = 0x60;
        const TAG_MASK: u32 = 0x3fff;
        const FOLD_MASK: u32 = 0x01ff_c000;
        #[inline(always)]
        unsafe fn rd(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(p: u32, v: u32) {
            unsafe { (p as *mut u32).write_unaligned(v) }
        }
        let obj: u32 = lf_checker_rt::callee_cdecl!(1, u32, ALLOC_SIZE, 0u32);
        let obj = if obj == 0 {
            0u32
        } else {
            let mut tag = rd(obj.wrapping_add(4));
            wr(obj, lf_checker_rt::relocated(BASE_VT));
            tag ^= rd(lf_checker_rt::relocated(COUNTER));
            tag &= TAG_MASK;
            wr(obj.wrapping_add(4), rd(obj.wrapping_add(4)) ^ tag);
            let c = lf_checker_rt::relocated(COUNTER);
            wr(c, rd(c).wrapping_add(1));
            wr(obj.wrapping_add(8), a1);
            wr(obj, lf_checker_rt::relocated(FINAL_VT));
            wr(obj.wrapping_add(0x0c), rd(a2));
                lf_checker_rt::callee_thiscall!(4, u32, obj.wrapping_add(0x10), a3);
                wr(obj.wrapping_add(0x50), rd(a4));
            obj
        };
        let vt = rd(obj);
        let slot = rd(vt.wrapping_add(8));
        let handler: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let r1 = handler(obj) as i32;
        let e = r1 % 16;
        let s = (16 - e) % 16;
        let r2 = handler(obj) as i32;
        let q = r2.wrapping_add(s) / 16;
        let m = rd(obj.wrapping_add(4));
        let folded = ((q as u32).wrapping_shl(14) ^ m) & FOLD_MASK;
        wr(obj.wrapping_add(4), m ^ folded);
        folded
    }
});
