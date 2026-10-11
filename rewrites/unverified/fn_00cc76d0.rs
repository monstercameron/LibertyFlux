// original: 0x00CC76D0 ARTFeedbackInterfaceGta::vf0

/// Initializes the feedback interface through its base initializer, then runs
/// the optional setup helper when the low bit of `flags` is set. The object
/// pointer is both the thiscall receiver and the value returned in EAX; the
/// one stack argument is a bit field, and the method removes that word before
/// returning. The optional helper receives the object pointer as one cdecl
/// stack argument. Callee effects are modeled by the proof contract.
lf_checker_rt::export!(thiscall, rw_00cc76d0(this: u32, flags: u32) -> u32 {
    const BASE_INITIALIZER: u32 = 1;
    const OPTIONAL_SETUP: u32 = 2;
    let _ = lf_checker_rt::callee_thiscall!(BASE_INITIALIZER, u32, this);
    if flags & 1 != 0 {
        let _ = lf_checker_rt::callee_cdecl!(OPTIONAL_SETUP, u32, this);
    }
    this
});
