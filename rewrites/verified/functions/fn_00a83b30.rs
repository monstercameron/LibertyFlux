// original: 0x00A83B30 CDeadInfo::append_drowned_flag

/// append drowned flag. Read the task-info object's recorded value and forward it with the matching static label through the checker-intercepted logger.
lf_checker_rt::export!(thiscall, rw_00a83b30(this: u32, logger_context: u32) -> () {
    const FORMAT: u32 = 0x00EA0B54;
    const FALSE_TEXT: u32 = 0x00EA0B4C;
    const TRUE_TEXT: u32 = 0x00EA0B44;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this, logger_context);
    let value = unsafe { core::ptr::read(this.wrapping_add(0x18) as *const u8) };
    let text = if value != 0 { TRUE_TEXT } else { FALSE_TEXT };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT), lf_checker_rt::relocated(text));
});
