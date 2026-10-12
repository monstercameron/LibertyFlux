// original: 0x008773F0 rage::crmfAngle::vf0

/// Scalar-deleting destructor for this class's base object.
///
/// Install this class's vtable. When the low bit of `flags` is set, read the
/// thread manager from TLS slot zero and call its virtual release method at
/// slot `+0x0c`, passing the manager as ECX and this object as the stack
/// argument. Return the original object pointer. The object pointer is
/// non-null on every completed original trial because the vtable store is
/// unconditional.
///
/// Original calling convention: thiscall with one stack flag word; the callee
/// removes that word.
lf_checker_rt::export!(thiscall, rw_008773f0(this: u32, flags: u32) -> u32 {
    const CLASS_VTABLE: u32 = 0x00ed338c;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(CLASS_VTABLE));

    if flags & 1 != 0 {
        const TLS_MANAGER_FIELD: u32 = 8;
        const RELEASE_SLOT: u32 = 0x0c;
        let thread_state = lf_checker_rt::tls_slot(0);
        let manager = ((thread_state.wrapping_add(TLS_MANAGER_FIELD)) as *const u32).read_unaligned();
        let vtable = (manager as *const u32).read_unaligned();
        let target = ((vtable.wrapping_add(RELEASE_SLOT)) as *const u32).read_unaligned();
        let release: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let _release_result = release(manager, this);
    }
    }
    this
});
