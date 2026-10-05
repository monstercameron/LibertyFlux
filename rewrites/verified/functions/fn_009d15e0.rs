// original: 0x009D15E0 dual_gate_append (proposed)
//
/// Appends elements that pass two virtual gates.
///
/// For each of the `count` (`[this+0x18]`, signed) elements at `[this+0x14]`,
/// calls virtual slot `+0x28` of `[this+4]` with
/// `(obj, A0, A4, A8, 2, 1, elem)`; a negative (signed) answer skips the
/// element. Survivors face slot `+0x30` with `(obj, A0, A4, A8, AC, elem)`;
/// a negative answer skips them too. The rest are appended through the helper
/// (callee 3 on `arg + 0x14`; the appended word is saved-register garbage the
/// original picks up from its own frame, so the argument is skipped without
/// a snapshot and unobserved on both sides). The virtual calls load their
/// slots through the fabricated object exactly like the original. Note the
/// original also overwrites its incoming arg slot mid-loop (write-only), so
/// the stack-word check is off. Returns nothing meaningful. Thiscall, one
/// pointer argument.
lf_checker_rt::export!(thiscall, rw_009D15E0(this: u32, arg: u32) -> u32 {
    unsafe {
        const OBJ: u32 = 0x04;
        const ARR: u32 = 0x14;
        const COUNT: u32 = 0x18;
        const TARGET: u32 = 0x14;
        const APPEND: u32 = 3;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        type V7 = extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
        type V6 = extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32;
        let count = rd(this.wrapping_add(COUNT));
        if (count as i32) <= 0 {
            return 0;
        }
        let arr = rd(this.wrapping_add(ARR));
        let obj = rd(this.wrapping_add(OBJ));
        let vtab = rd(obj);
        let f1: V7 = core::mem::transmute(rd(vtab.wrapping_add(0x28)) as usize);
        let f2: V6 = core::mem::transmute(rd(vtab.wrapping_add(0x30)) as usize);
        let a0 = rd(arg);
        let a4 = rd(arg.wrapping_add(4));
        let a8 = rd(arg.wrapping_add(8));
        let ac = rd(arg.wrapping_add(0xc));
        let mut i = 0u32;
        while (i as i32) < (count as i32) {
            let elem = rd(arr.wrapping_add(i.wrapping_mul(4)));
            let s1 = f1(vtab, obj, a0, a4, a8, 2, 1, elem);
            if (s1 as i32) < 0 {
                i += 1;
                continue;
            }
            let s2 = f2(vtab, obj, a0, a4, a8, ac, elem);
            if (s2 as i32) < 0 {
                i += 1;
                continue;
            }
            let mut slot = 0u32;
            let _: u32 = lf_checker_rt::callee_thiscall!(
                APPEND, u32, arg.wrapping_add(TARGET), (&mut slot as *mut u32) as u32);
            i += 1;
        }
        0
    }
});
