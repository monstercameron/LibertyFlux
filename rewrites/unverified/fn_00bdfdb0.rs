// original: 0x00BDFDB0 CTaskSimpleDuckToggle::vf0
/// Scalar deleting destructor for `CTaskSimpleDuckToggle` (vtable slot 0).
///
/// Runs the class destructor as a thiscall on `this` (always), then frees
/// the object only when bit 0 of `flags` is set: the allocator object is
/// loaded from the global at `ALLOCATOR` and its delete routine runs as a
/// thiscall with the allocator in ECX and `this` as the single stack word.
/// Returns `this` unchanged. Original is thiscall with one stack word and
/// pops 4 bytes.
lf_checker_rt::export!(thiscall, rw_00bdfdb0(this: u32, flags: u32) -> u32 {
    const DTOR: u32 = 1;
    const DELETE: u32 = 2;
    const ALLOCATOR: u32 = 0x0167E2A0;
    const DELETE_BIT: u32 = 1;
    let _ = lf_checker_rt::callee_thiscall!(DTOR, u32, this);
    if flags & DELETE_BIT != 0 {
        let allocator = unsafe { lf_checker_rt::global::<u32>(ALLOCATOR).read() };
        let _ = lf_checker_rt::callee_thiscall!(DELETE, u32, allocator, this);
    }
    this
});
