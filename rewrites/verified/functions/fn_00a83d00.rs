// original: 0x00A83D00 CPlayRandomAmbientsInfo::append_base_animation_flag

/// append base animation flag. Read the task-info object's recorded value and forward it with the matching static label through the checker-intercepted logger.
lf_checker_rt::export!(thiscall, rw_00a83d00(this: u32, logger_context: u32) -> () {
    const FORMAT: u32 = 0x00EA0A68;
    const FALSE_TEXT: u32 = 0x00EA0A60;
    const TRUE_TEXT: u32 = 0x00EA0A58;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this, logger_context);
    let value = unsafe { core::ptr::read(this.wrapping_add(0x18) as *const u8) };
    let text = if value != 0 { TRUE_TEXT } else { FALSE_TEXT };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT), lf_checker_rt::relocated(text));
});
