// original: 0x00CA2D20 CPedDamageResponse::vf0

/// Install the CPedDamageResponse vtable in the receiver. If the low bit of the flags argument is set, call the release-side initializer with the receiver. Return the receiver pointer in EAX and remove the stack flags word. The vtable value is represented by a named 32-bit symbol constant because the object layout in this build uses 32-bit vtable pointers.
lf_checker_rt::export!(thiscall, rw_00ca2d20(this: u32, flags: u32) -> u32 {
    const PED_DAMAGE_RESPONSE_VA: u32 = 0x00ED7418;
    const DELETE_FLAG: u32 = 1;
    const RELEASE_INITIALIZER: u32 = 1;
    let vtable = lf_checker_rt::relocated(PED_DAMAGE_RESPONSE_VA);
    unsafe { (this as *mut u32).write_unaligned(vtable); }
    if flags & DELETE_FLAG != 0 {
        let _result = lf_checker_rt::callee_cdecl!(RELEASE_INITIALIZER, u32, this);
    }
    this
});
