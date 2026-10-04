// original: 0x00cf5430 climb_task_controller_reset (proposed)

/// Resets a climb task's controller: resets the controller at `+0xa80` with
/// argument 0, and unless the task already has a live ladder link (non-null
/// word at `+0x6c` whose flag byte at `+0xe` is set) zeroes the controller
/// through the secondary reset and clears its word at `+8`. Then stamps
/// 0x461c3c00 at `+0x288` of the block at `+0x224`, clears bit 1 of the word
/// at `+0x46` of the state at `+0x10` when present, and runs the 1000-step
/// initialiser, returning its result.
///
/// Original: 0x00cf5430 (thiscall: ecx holds the object, one stack word).
lf_checker_rt::export!(thiscall, rw_00cf5430(this: u32, target: u32) -> u32 {
    unsafe {
        const RESET_CALLEE: u32 = 1;
        const ZERO_CALLEE: u32 = 2;
        const INIT_CALLEE: u32 = 3;
        const STAMP: u32 = 0x461c_3c00;
        let ctl = ((target + 0xa80) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(RESET_CALLEE, u32, ctl, 0);
        let link = ((target + 0x6c) as *const u32).read_unaligned();
        let live = link != 0 && ((link + 0xe) as *const u8).read() != 0;
        if !live {
            let ctl2 = ((target + 0xa80) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(ZERO_CALLEE, u32, ctl2, 0, 0, 0);
            let ctl3 = ((target + 0xa80) as *const u32).read_unaligned();
            ((ctl3 + 8) as *mut u32).write_unaligned(0);
        }
        let block = ((target + 0x224) as *const u32).read_unaligned();
        ((block + 0x288) as *mut u32).write_unaligned(STAMP);
        let state = ((this + 0x10) as *const u32).read_unaligned();
        if state != 0 {
            let w = ((state + 0x46) as *const u16).read_unaligned();
            ((state + 0x46) as *mut u16).write_unaligned(w & 0xfffd);
        }
        lf_checker_rt::callee_stdcall!(INIT_CALLEE, u32, target)
    }
});
