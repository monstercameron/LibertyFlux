// original: 0x009D1650 fetch_clamp_append (proposed)
//
/// Fetches a value per element through a virtual call, clamps and appends.
///
/// For each of the `count` (`[this+0x24]`, signed) elements at `[this+0x20]`,
/// calls virtual slot `+0x2c` of `[this+4]` with
/// `(obj, B0, B4, BC, B10, elem, &slot)` where the slot first holds
/// `arg` (observed via the call snapshot; the struct's second word is
/// never-written caller-stack garbage, unobserved on both sides) and receives the fetched
/// value (scripted). A negative (signed) status skips the element.
/// Survivors append the element to `arg + 0x20` (through a frame slot the
/// original first clobbers with the element), clamp the fetched value down to
/// `[this+0x54] + 1` with an UNSIGNED minimum, and append the clamped value
/// to `arg + 0x2c` (both appends through callee 2, values observed via
/// snapshots). The virtual call loads its slot through the fabricated object
/// exactly like the original. The original reuses its incoming arg slot as
/// the fetch slot, so the stack-word check is off. Returns nothing
/// meaningful. Thiscall, one pointer-sized word argument.
lf_checker_rt::export!(thiscall, rw_009D1650(this: u32, arg: u32) -> u32 {
    unsafe {
        const OBJ: u32 = 0x04;
        const ARR: u32 = 0x20;
        const COUNT: u32 = 0x24;
        const LIMIT: u32 = 0x54;
        const APPEND: u32 = 2;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        type V7 = extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
        let count = rd(this.wrapping_add(COUNT));
        if (count as i32) <= 0 {
            return 0;
        }
        let arr = rd(this.wrapping_add(ARR));
        let obj = rd(this.wrapping_add(OBJ));
        let vtab = rd(obj);
        let f: V7 = core::mem::transmute(rd(vtab.wrapping_add(0x2c)) as usize);
        let bound = rd(this.wrapping_add(LIMIT)).wrapping_add(1);
        let b0 = rd(arg);
        let b4 = rd(arg.wrapping_add(4));
        let bc = rd(arg.wrapping_add(0xc));
        let b10 = rd(arg.wrapping_add(0x10));
        let mut i = 0u32;
        // The fetch slot keeps its value across iterations (the original
        // reuses its incoming arg slot), so each call observes the previous
        // call's written result.
        let mut slot = arg;
        while (i as i32) < (count as i32) {
            let elem = rd(arr.wrapping_add(i.wrapping_mul(4)));
            let st = f(obj, obj, b0, b4, bc, b10, elem, (&mut slot as *mut u32) as u32);
            if (st as i32) < 0 {
                i += 1;
                continue;
            }
            let mut t = elem;
            let _: u32 = lf_checker_rt::callee_thiscall!(
                APPEND, u32, arg.wrapping_add(0x20), (&mut t as *mut u32) as u32);
            let mut v = slot;
            if v > bound {
                v = bound;
            }
            slot = v;
            let _: u32 = lf_checker_rt::callee_thiscall!(
                APPEND, u32, arg.wrapping_add(0x2c), (&mut v as *mut u32) as u32);
            i += 1;
        }
        0
    }
});
