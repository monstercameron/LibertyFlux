// original: 0x00697fe0 mover_maybe_init (proposed)

/// Run the guarded constructor on `obj` unless it is null.
///
/// The first stack word is ignored. When the second (`obj`) is non-zero the
/// helper is invoked with it both as the object pointer and as its single
/// stack argument; a null `obj` takes no call. No result.
///
/// Original: cdecl, two stack words, caller cleans.
lf_checker_rt::export!(cdecl, rw_00697fe0(_ignored: u32, obj: u32) -> u32 {
    unsafe {
        const CALLEE: u32 = 1;
        if obj != 0 {
            let _ = lf_checker_rt::callee_thiscall!(CALLEE, u32, obj, obj);
        }
        0
    }
});
