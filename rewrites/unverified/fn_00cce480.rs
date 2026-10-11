// original: 0x00CCE480 CTaskSimpleDie::vf17

/// Passing stage for the task's nonzero mode byte. It forwards the event
/// pointer and constant mode to the preparation helper, then forwards the
/// task's +0x30 float bits to the follow-up helper. The original overwrites
/// only EAX's low byte with one after the follow-up returns. The zero-mode
/// branches and the remaining state updates are not part of this stage.
///
/// ABI: thiscall with the task in ECX and one incoming stack word.
pub(crate) unsafe fn run(task: u32, event: u32) -> u32 {
    const MODE: u32 = 0x34;
    const MODE_ENABLED: u8 = 1;
    const TASK_FLOAT: u32 = 0x30;
    const PREPARE: u32 = 1;
    const FOLLOW_UP: u32 = 2;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        if read_u8(task.wrapping_add(MODE)) == 0 {
            return 0;
        }
        let _prepare_result = lf_checker_rt::callee_thiscall!(PREPARE, u32, event, 1u32);
        let parameter_bits = read_u32(task.wrapping_add(TASK_FLOAT));
        let follow_up_result =
            lf_checker_rt::callee_thiscall!(FOLLOW_UP, u32, task, parameter_bits);
        (follow_up_result & 0xffff_ff00) | u32::from(MODE_ENABLED)
    }
}

pub(crate) unsafe fn run_mutant(task: u32, event: u32) -> u32 {
    const MODE: u32 = 0x34;
    const TASK_FLOAT: u32 = 0x30;
    const PREPARE: u32 = 1;
    const FOLLOW_UP: u32 = 2;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        if read_u8(task.wrapping_add(MODE)) == 0 {
            return 0;
        }
        let _prepare_result = lf_checker_rt::callee_thiscall!(PREPARE, u32, event, 1u32);
        let parameter_bits = read_u32(task.wrapping_add(TASK_FLOAT));
        // Wrong version: does not replace AL with one after the second call.
        lf_checker_rt::callee_thiscall!(FOLLOW_UP, u32, task, parameter_bits)
    }
}

lf_checker_rt::export!(thiscall, rw_00cce480(task: u32, event: u32) -> u32 {
    unsafe { run(task, event) }
});
