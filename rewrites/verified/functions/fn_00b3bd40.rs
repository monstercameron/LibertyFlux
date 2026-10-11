// original: 0x00B3BD40 resolve_handle_or_apply_context
/// Resolve one handle with the first helper. The sentinel value is returned
/// directly; all other results are passed with the configured context word
/// to a second helper whose return value becomes the function result.
lf_checker_rt::export!(cdecl, rw_00b3bd40(handle: u32) -> u32 {
    unsafe {
        const CONTEXT_WORD: u32 = 0x012b4138;
        const NOT_FOUND: u32 = u32::MAX;
        let resolved = lf_checker_rt::callee_cdecl!(1, u32, handle);
        if resolved == NOT_FOUND {
            resolved
        } else {
            let context = lf_checker_rt::global::<u32>(CONTEXT_WORD).read_unaligned();
            lf_checker_rt::callee_cdecl!(2, u32, resolved, context)
        }
    }
});
