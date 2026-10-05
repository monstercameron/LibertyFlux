// original: 0x008C7E10 stream_log_forward
/// Resolve `tag` to its message id and forward it to the log callee.
///
/// The resolve callee maps the tag, then the log callee receives the
/// caller's first word, the log-format address and the resolved id.
/// Returns nothing. Original: cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_008c7e10(first: u32, tag: u32) -> u32 {
    unsafe {
        const RESOLVE_CALLEE: u32 = 1;
        const LOG_CALLEE: u32 = 2;
        const LOG_FORMAT_FILE_VA: u32 = 0x00E80404;
        let id: u32 = lf_checker_rt::callee_cdecl!(
            RESOLVE_CALLEE, u32, tag);
        let format = lf_checker_rt::relocated(LOG_FORMAT_FILE_VA);
        lf_checker_rt::callee_cdecl!(LOG_CALLEE, u32, first, format, id);
        0
    }
});
