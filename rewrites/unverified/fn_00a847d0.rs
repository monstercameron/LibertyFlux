// original: 0x00A847D0 CDieInfo::append_network_sequence_id

/// append network sequence id. Read the task-info object's recorded value and forward it with the matching static label through the checker-intercepted logger.
lf_checker_rt::export!(thiscall, rw_00a847d0(this: u32, logger_context: u32) -> () {
    const FORMAT: u32 = 0x00EA0524;
    let value = unsafe { core::ptr::read_unaligned(this.wrapping_add(0x14) as *const u32) };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT), value);
});
