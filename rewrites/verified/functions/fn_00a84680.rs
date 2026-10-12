// original: 0x00A84680 CStandUpTaskInfo::append_animation_ids

/// append animation ids. Read the task-info object's recorded value and forward it with the matching static label through the checker-intercepted logger.
lf_checker_rt::export!(thiscall, rw_00a84680(this: u32, logger_context: u32) -> () {
    const FORMAT_0: u32 = 0x00EA03A0;
    const FORMAT_1: u32 = 0x00EA03B4;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this, logger_context);
    let value_0 = unsafe { core::ptr::read_unaligned(this.wrapping_add(0x18) as *const u32) };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_0), value_0);
    let value_1 = unsafe { core::ptr::read_unaligned(this.wrapping_add(0x1C) as *const u32) };
    let _ = lf_checker_rt::callee_cdecl!(3, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_1), value_1);
});
