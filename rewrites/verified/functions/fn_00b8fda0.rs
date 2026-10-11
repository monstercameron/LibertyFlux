// original: 0x00B8FDA0 NativeImpl_GET_STRING_FROM_HASH_KEY

lf_checker_rt::export!(cdecl, rw_00b8fda0(context: u32) -> u32 {
    const FORMAT_HELPER: u32 = 1;
    const RESOLVE_HASH_HELPER: u32 = 2;
    const FORMAT_LINE_HELPER: u32 = 3;
    const COOKIE_CHECK_HELPER: u32 = 4;
    const FORMAT_STRING: u32 = 0x00EB5524;
    const TEXT_CONTEXT: u32 = 0x0116BFF0;

    let mut local_text = [0u32; 4];
    let text_pointer = local_text.as_mut_ptr() as u32;

    let _format_result = lf_checker_rt::callee_cdecl!(
        FORMAT_HELPER,
        u32,
        text_pointer,
        lf_checker_rt::relocated(FORMAT_STRING),
        context,
    );
    let resolved_hash = lf_checker_rt::callee_thiscall!(
        RESOLVE_HASH_HELPER,
        u32,
        lf_checker_rt::relocated(TEXT_CONTEXT),
        context,
        text_pointer,
    );
    let _line_result = lf_checker_rt::callee_cdecl!(
        FORMAT_LINE_HELPER,
        u32,
        resolved_hash,
    );
    let _cookie_result = lf_checker_rt::callee_thiscall!(
        COOKIE_CHECK_HELPER,
        u32,
        0,
    );
    0
});
