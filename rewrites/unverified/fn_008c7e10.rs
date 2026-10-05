// original: 0x008C7E10 stream_log_forward
/// Resolve `tag` to its message id and forward it to the log callee.
///
/// The resolve callee maps the tag, then the log callee receives the tag,
/// the log-format constant and the resolved id. The first caller word is
/// ignored. Returns nothing. Original: cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_008c7e10(_unused: u32, tag: u32) -> u32 {
    unsafe {
        const RESOLVE_CALLEE: u32 = 1;
        const LOG_CALLEE: u32 = 2;
        const LOG_FORMAT: u32 = 0x00E80404;
        let id: u32 = lf_checker_rt::callee_cdecl!(
            RESOLVE_CALLEE, u32, tag);
        lf_checker_rt::callee_cdecl!(LOG_CALLEE, u32, tag, LOG_FORMAT, id);
        0
    }
});
