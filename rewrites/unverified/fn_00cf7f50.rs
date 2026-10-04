// original: 0x00cf7f50 climb_task_target_attach (proposed)

/// Attaches a climb task to its target object: runs two target-registration
/// helpers with the object, resets the controller at `+0xa80` with argument
/// 0, sets bit 0 and clears bit 1 of the object's word at `+0x26c`, and when
/// the flag byte at `+0xce` is set runs the ladder finaliser on the block at
/// `+0x3c0`, returning its result (otherwise the updated word).
///
/// Original: 0x00cf7f50 (thiscall: ecx holds the object, one stack word).
lf_checker_rt::export!(thiscall, rw_00cf7f50(this: u32, target: u32) -> u32 {
    unsafe {
        const REGISTER_A: u32 = 1;
        const REGISTER_B: u32 = 2;
        const RESET_CALLEE: u32 = 3;
        const FINISHER: u32 = 4;
        lf_checker_rt::callee_thiscall!(REGISTER_A, u32, this, target);
        lf_checker_rt::callee_thiscall!(REGISTER_B, u32, this, target);
        let ctl = ((target + 0xa80) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(RESET_CALLEE, u32, ctl, 0);
        let w = ((target + 0x26c) as *const u32).read_unaligned();
        let updated = (w & 0xffff_fffd) | 1;
        ((target + 0x26c) as *mut u32).write_unaligned(updated);
        let flag = ((this + 0xce) as *const u8).read();
        if flag != 0 {
            lf_checker_rt::callee_thiscall!(FINISHER, u32, target.wrapping_add(0x3c0))
        } else {
            updated
        }
    }
});
