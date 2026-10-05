// original: 0x0069BB10 rage::crAnimChannelCurveFloat::init_guarded

/// Runs the curve-float mapper unless the object pointer is null.
///
/// The stack arguments are the channel object and the mapper's argument. A
/// null object returns immediately (whatever `eax` held on entry, which no
/// rewrite can reproduce, so the contract only exercises live objects); a
/// live object is forwarded to the mapper (callee 1, thiscall: object,
/// argument) whose answer is the return value in `eax`.
///
/// Original: 0x0069BB10 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_0069BB10(obj: u32, arg1: u32) -> u32 {
    unsafe {
        const MAP: u32 = 1;
        if obj == 0 {
            return 0; // placeholder: original returns entry-eax residue (unverifiable, uncovered)
        }
        lf_checker_rt::callee_thiscall!(MAP, u32, obj, arg1)
    }
});
