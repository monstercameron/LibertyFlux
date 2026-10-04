// original: 0x00D4E060 CTaskSimpleTakeOffHelmet::vf5

// Event handler (vf5): vetoes stale events through the interested object, else advances.
///
/// When the second stack argument is below 1 (signed), the third argument is
/// required non-null and is asked through its vtable slot at `+0x18`
/// (intercepted callee 1, register-indirect in the original); a zero answer
/// or a null object yields 0. Then, when the word at `+0x10` is non-zero and
/// the second argument is 2, the float -1000.0 goes with `this` to intercepted
/// callee 2. (The original passes whatever ECX holds there, but that site is
/// only reachable with the second argument equal to 2, which always takes the
/// non-veto path where ECX is still `this`; the veto path can never reach it
/// with the argument below 1.) Finally the first argument goes with `this` to
/// intercepted callee 3, 4 is stored at `+0x14`, and the result is 1.
///
/// Original: 0x00D4E060 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00d4e060(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const NEG_THOUSAND_BITS: u32 = 0xC47A0000; // -1000.0f
        const VETO: u32 = 1;
        const PUSH: u32 = 2;
        const ADVANCE: u32 = 3;
        const VETO_SLOT: u32 = 0x18;
        if (a1 as i32) < 1 {
            if a2 == 0 {
                return 0;
            }
            let vt = (a2 as *const u32).read_unaligned();
            let addr = ((vt + VETO_SLOT) as *const u32).read_unaligned();
            let veto: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(addr as usize);
            if veto(a2) as u8 == 0 {
                return 0;
            }
        }
        if ((this + 0x10) as *const u32).read_unaligned() != 0 && a1 == 2 {
            lf_checker_rt::callee_thiscall!(PUSH, u32, this, NEG_THOUSAND_BITS);
        }
        lf_checker_rt::callee_thiscall!(ADVANCE, u32, this, a0);
        ((this + 0x14) as *mut u32).write_unaligned(4);
        1
    }
});
