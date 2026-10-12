// original: 0x00BDFD60 CTaskList::vf0
/// Scalar deleting destructor for `CTaskList` (vtable slot 0).
///
/// Runs the class destructor as a thiscall on `this` (always), then frees
/// the object with the cdecl delete routine only when bit 0 of `flags` is
/// set, passing `this` as the single stack word. Returns `this` unchanged.
/// Original is thiscall with one stack word and pops 4 bytes.
lf_checker_rt::export!(thiscall, rw_00bdfd60(this: u32, flags: u32) -> u32 {
    const DTOR: u32 = 1;
    const DELETE: u32 = 2;
    const DELETE_BIT: u32 = 1;
    let _ = lf_checker_rt::callee_thiscall!(DTOR, u32, this);
    if flags & DELETE_BIT != 0 {
        let _ = lf_checker_rt::callee_cdecl!(DELETE, u32, this);
    }
    this
});
