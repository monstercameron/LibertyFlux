// original: 0x00b4bae0 group_initialize

/// Initialize a group object's member subobject, flags, and result field.
/// The thiscall object is in ECX. The member initializer is called at +8;
/// after the field resets, a zero-argument helper supplies the value stored
/// at +0x50. The function returns the original object pointer.
lf_checker_rt::export!(thiscall, rw_00B4BAE0(object: u32) -> u32 {
    const MEMBER_SUBOBJECT: u32 = 0x08;
    const RESULT_FIELD: u32 = 0x50;
    const STATUS_BYTE: u32 = 0x54;
    const FLAGS_FIELD: u32 = 0x56;
    const AUX_FIELD: u32 = 0x58;
    const CLEAR_LOW_FLAG: u16 = 0xFFFE;
    const INITIAL_FLAGS: u16 = 0x0006;

    // The callee is intercepted by the contract at the original call site.
    let _member_result = lf_checker_rt::callee_thiscall!(1, u32,
        object.wrapping_add(MEMBER_SUBOBJECT));
    let flags_ptr = object.wrapping_add(FLAGS_FIELD) as *mut u16;
    // SAFETY: the supplied group object covers the initialized fields.
    let flags = unsafe { flags_ptr.read_unaligned() };
    let initialized_flags = (flags & CLEAR_LOW_FLAG) | INITIAL_FLAGS;
    unsafe {
        (object.wrapping_add(RESULT_FIELD) as *mut u32).write_unaligned(0);
        (object.wrapping_add(STATUS_BYTE) as *mut u8).write(0);
        flags_ptr.write_unaligned(initialized_flags);
        (object.wrapping_add(AUX_FIELD) as *mut u32).write_unaligned(0);
        (object.wrapping_add(MEMBER_SUBOBJECT) as *mut u32)
            .write_unaligned(object);
        (object as *mut u32).write_unaligned(0);
    }

    let result = lf_checker_rt::callee_cdecl!(2, u32,);
    // SAFETY: the result field is within the object described above.
    unsafe { (object.wrapping_add(RESULT_FIELD) as *mut u32).write_unaligned(result) };
    object
});
