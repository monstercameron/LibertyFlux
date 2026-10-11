// original: 0x00CD9A50 CPedIntelligenceFactory::vf0

/// Destroy the ped intelligence factory's object portion and optionally
/// release its storage. `this` is the factory object and `flags` is the
/// deleting-destructor flag word; bit zero controls the release call. The
/// class destructor runs first, and the function returns the same object
/// pointer.
///
/// Calling convention: thiscall with one 32-bit stack argument. The
/// conditional release helper is called as cdecl with the object pointer.
lf_checker_rt::export!(thiscall, rw_00cd9a50(this: u32, flags: u32) -> u32 {
    const BASE_DESTRUCTOR: u32 = 1;
    const OBJECT_RELEASE: u32 = 2;
    const DELETE_TO_POOL: u32 = 1;

    let _ = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this);
    if flags & DELETE_TO_POOL != 0 {
        let _ = lf_checker_rt::callee_cdecl!(OBJECT_RELEASE, u32, this);
    }
    this
});
