// original: 0x00DE7750 UIMontageEditorLayout::vf2

/// Reinitialize this layout through its secondary constructor, then inspect
/// bit zero of the incoming flags. When set, call the native message handler
/// with `this`; return `this` in all cases. This `thiscall` method removes its
/// one stack argument on return.
lf_checker_rt::export!(thiscall, rw_00DE7750(this: u32, delete_flags: u32) -> u32 {
    const CALLEE_REINITIALIZE: u32 = 1;
    const CALLEE_SEND_MESSAGE: u32 = 2;
    let _ = lf_checker_rt::callee_thiscall!(CALLEE_REINITIALIZE, u32, this);
    if delete_flags & 1 != 0 {
        let _ = lf_checker_rt::callee_cdecl!(CALLEE_SEND_MESSAGE, u32, this);
    }
    this
});
