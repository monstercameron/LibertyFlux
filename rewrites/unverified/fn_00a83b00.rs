// original: 0x00A83B00 CComplexSlideIntoCoverInfo::append_cover_animation

/// append cover animation. Read the task-info object's recorded value and forward it with the matching static label through the checker-intercepted logger.
lf_checker_rt::export!(thiscall, rw_00a83b00(this: u32, logger_context: u32) -> () {
    const FORMAT: u32 = 0x00EA0744;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this, logger_context);
    let value = unsafe { core::ptr::read_unaligned(this.wrapping_add(0x18) as *const u32) };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT), value);
});
