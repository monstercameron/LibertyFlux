// original: 0x00A83B70 CJackTaskInfo::append_jack_attack_flag

/// append jack attack flag. Read the task-info object's recorded value and forward it with the matching static label through the checker-intercepted logger.
lf_checker_rt::export!(thiscall, rw_00a83b70(this: u32, logger_context: u32) -> () {
    const FORMAT: u32 = 0x00EA070C;
    const FALSE_TEXT: u32 = 0x00EA0704;
    const TRUE_TEXT: u32 = 0x00EA06FC;
    let value = unsafe { core::ptr::read(this.wrapping_add(0x14) as *const u8) };
    let text = if value != 0 { TRUE_TEXT } else { FALSE_TEXT };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT), lf_checker_rt::relocated(text));
});
