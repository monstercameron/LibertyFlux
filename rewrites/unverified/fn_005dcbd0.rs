// original: 0x005DCBD0 CPedIntelligenceFactoryNY::vf1

/// Allocate and initialize a pedestrian intelligence factory object.
///
/// The method requests an object from the shared factory helper. A null
/// result returns zero. Otherwise it initializes the new object with the
/// caller's value, installs its primary vtable, constructs the event handler
/// at offset 0x2f0 and installs that subobject's vtable, constructs the event
/// scanner at offset 0x330, then installs its vtable and returns the object.
/// The caller value is forwarded unchanged to the first two constructors.
lf_checker_rt::export!(thiscall, rw_005dcbd0(_factory: u32, variant: u32) -> u32 {
    const PRIMARY_VTABLE: u32 = 0x00FE12EC;
    const HANDLER_OFFSET: u32 = 0x2F0;
    const HANDLER_VTABLE: u32 = 0x00FE1184;
    const SCANNER_OFFSET: u32 = 0x330;
    const SCANNER_VTABLE: u32 = 0x00FE12D8;

    let object = lf_checker_rt::callee_cdecl!(1, u32,);
    if object == 0 {
        return 0;
    }

    let _ = lf_checker_rt::callee_thiscall!(2, u32, object, variant);
    unsafe {
        (object as *mut u32).write_unaligned(lf_checker_rt::relocated(PRIMARY_VTABLE));
    }

    let _ = lf_checker_rt::callee_thiscall!(
        3,
        u32,
        object.wrapping_add(HANDLER_OFFSET),
        variant,
    );
    unsafe {
        (object.wrapping_add(HANDLER_OFFSET) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(HANDLER_VTABLE));
    }

    let _ = lf_checker_rt::callee_thiscall!(
        4,
        u32,
        object.wrapping_add(SCANNER_OFFSET),
    );
    unsafe {
        (object.wrapping_add(SCANNER_OFFSET) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(SCANNER_VTABLE));
    }

    object
});
