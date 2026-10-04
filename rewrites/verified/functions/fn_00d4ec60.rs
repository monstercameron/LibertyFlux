// original: 0x00D4EC60 CTaskSimplePutDownObject::vf0

/// Scalar deleting destructor of CTaskSimplePutDownObject.
///
/// Runs the class destructor on `this` (intercepted callee 1, thiscall with
/// no stack arguments), then, when the low bit of `flags` is set, frees the
/// object through the game allocator whose pointer lives in a global slot
/// (intercepted callee 2, thiscall taking the object pointer). Returns `this`
/// in both cases. The destructor's own return value is discarded, and the
/// allocator pointer is read only on the freeing path.
///
/// Original: 0x00D4EC60 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d4ec60(this: u32, flags: u32) -> u32 {
    unsafe {
        const ALLOCATOR_SLOT: u32 = 0x0167E2A0;
        const DTOR: u32 = 1;
        const DELETE: u32 = 2;
        lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if flags & 1 != 0 {
            let alloc = lf_checker_rt::global::<u32>(ALLOCATOR_SLOT).read();
            lf_checker_rt::callee_thiscall!(DELETE, u32, alloc, this);
        }
        this
    }
});
