// original: 0x00DE7030 UITimeOverview::vf2

/// Reinitialize the time-overview object's secondary state, then inspect bit
/// zero of the incoming delete flags. When set, notify the native message
/// handler with `this`; finally return the object pointer. The `thiscall`
/// method removes its one stack argument on return.
lf_checker_rt::export!(thiscall, rw_00DE7030(this: u32, delete_flags: u32) -> u32 {
    const CALLEE_REINITIALIZE: u32 = 1;
    const CALLEE_SEND_MESSAGE: u32 = 2;
    let _ = lf_checker_rt::callee_thiscall!(CALLEE_REINITIALIZE, u32, this);
    if delete_flags & 1 != 0 {
        let _ = lf_checker_rt::callee_cdecl!(CALLEE_SEND_MESSAGE, u32, this);
    }
    this
});
