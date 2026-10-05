// original: 0x0093f0d0 stream_selected_forward (proposed)

/// Tail-call the handler with the selected object, or return null.
///
/// Calls the selected-slot getter; a null object returns 0, otherwise the
/// handler callee runs with the object in `ecx` and its answer is the
/// result (the original reaches it with a tail jump; the rewrite calls it
/// through the same intercepted callee, which balances either way).
///
/// Original: 0x0093f0d0 (cdecl, no arguments; one direct + one tail callee).
lf_checker_rt::export!(cdecl, rw_0093f0d0() -> u32 {
    const SELECTED_GETTER: u32 = 1;
    const HANDLER: u32 = 2;
    unsafe {
        let obj: u32 = lf_checker_rt::callee_cdecl!(SELECTED_GETTER, u32,);
        if obj == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(HANDLER, u32, obj)
    }
});
