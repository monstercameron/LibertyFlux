// original: 0x005C32E0 string_holder_delete

/// Delete the string held by this object when its stored length is nonzero,
/// release its optional secondary resource through TLS, then release the
/// holder itself through the same thread-local destructor. The helper receives
/// an otherwise-unused stack word, returns `this`, and reads the holder's
/// pointer, secondary-state word, string pointer, and 16-bit length.
lf_checker_rt::export!(thiscall, rw_005c32e0(this: u32, unused_arg: u32) -> u32 {
    const SECONDARY_POINTER_OFFSET: u32 = 0;
    const SECONDARY_ENABLED_OFFSET: u32 = 6;
    const STRING_POINTER_OFFSET: u32 = 8;
    const STRING_LENGTH_OFFSET: u32 = 0xE;
    const TLS_ENV_POINTER_OFFSET: u32 = 8;
    const CLEANUP_VTABLE_RELEASE_SLOT_OFFSET: u32 = 12;
    const DELETE_STRING_CALLEE: u32 = 1;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn release_through_tls(resource: u32) {
        let environment = lf_checker_rt::tls_slot(0);
        let cleanup_object = unsafe { read_u32(environment + TLS_ENV_POINTER_OFFSET) };
        let vtable = unsafe { read_u32(cleanup_object) };
        let release = unsafe { read_u32(vtable + CLEANUP_VTABLE_RELEASE_SLOT_OFFSET) };
        let release_fn: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(release as usize) };
        let _ = release_fn(cleanup_object, resource);
    }

    let string_length = unsafe { read_u16(this + STRING_LENGTH_OFFSET) } as u32;
    if string_length != 0 {
        let string = unsafe { read_u32(this + STRING_POINTER_OFFSET) };
        let _: u32 = lf_checker_rt::callee_stdcall!(DELETE_STRING_CALLEE, u32, string, string_length);
    }

    if unsafe { read_u16(this + SECONDARY_ENABLED_OFFSET) } != 0 {
        let secondary = unsafe { read_u32(this + SECONDARY_POINTER_OFFSET) };
        if secondary != 0 {
            unsafe { release_through_tls(secondary) };
        }
    }

    unsafe { release_through_tls(this) };
    let _ = unused_arg;
    this
});
