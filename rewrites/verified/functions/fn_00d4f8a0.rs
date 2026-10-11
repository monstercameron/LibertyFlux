// original: 0x00D4F8A0 CTaskSimplePutDownObject::vf5
/// Advance the putdown task. Unless mode two is selected, a related task
/// at the third stack argument is queried; its signed result is compared with
/// the inclusive upper bound 46. A zero low-byte allow result ends the update;
/// a nonzero result continues to cleanup. The task link at `+0x1c` receives
/// a mode-dependent force value,
/// then both task and resource links are released and cleared. The resource
/// link is at `+0x14`; the completion byte is at `+0x2c`. The force is
/// passed as exact f32 bits on the stack. The function is thiscall with three
/// stack arguments; its return is a low-byte boolean.
lf_checker_rt::export!(thiscall, rw_00d4f8a0(this: u32, _unused: u32, mode: u32, related_task: u32) -> u8 {
    const MODE_WITH_STRONG_FORCE: u32 = 2;
    const MAX_QUERY_RESULT: i32 = 46;
    const TASK_LINK: u32 = 0x1c;
    const RESOURCE_LINK: u32 = 0x14;
    const READY_FLAG: u32 = 0x2c;
    const STRONG_FORCE_BITS: u32 = 0xc47a_0000;
    const DEFAULT_FORCE_BITS: u32 = 0xc0800000;
    const QUERY_RELATED_TASK: u32 = 1;
    const ALLOW_TASK: u32 = 2;
    const APPLY_FORCE: u32 = 3;
    const RELEASE_TASK: u32 = 4;
    const RELEASE_RESOURCE: u32 = 5;

    if mode != MODE_WITH_STRONG_FORCE && related_task != 0 {
        let query_result: u32 =
            lf_checker_rt::callee_thiscall!(QUERY_RELATED_TASK, u32, related_task);
        if (query_result as i32) <= MAX_QUERY_RESULT {
            let allowed: u8 = lf_checker_rt::callee_thiscall!(ALLOW_TASK, u8, related_task);
            if allowed == 0 {
                return 0;
            }
        }
    }

    let task_link = unsafe { ((this + TASK_LINK) as *const u32).read_unaligned() };
    if task_link != 0 {
        let force = if mode == MODE_WITH_STRONG_FORCE { STRONG_FORCE_BITS } else { DEFAULT_FORCE_BITS };
        let _: u32 = lf_checker_rt::callee_thiscall!(APPLY_FORCE, u32, task_link, force);
        let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE_TASK, u32, task_link, this);
        unsafe { ((this + TASK_LINK) as *mut u32).write_unaligned(0) };
    }

    let resource = unsafe { ((this + RESOURCE_LINK) as *const u32).read_unaligned() };
    if resource != 0 {
        let resource_slot = this + RESOURCE_LINK;
        let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE_RESOURCE, u32, resource, resource_slot);
        unsafe { ((resource_slot) as *mut u32).write_unaligned(0) };
    }

    unsafe { ((this + READY_FLAG) as *mut u8).write(1) };
    1
});
