// original: 0x005C3330 string_array_holder_delete

/// Release the stored string array when its 16-bit length is nonzero, then
/// release the optional secondary resource and this holder through the
/// current thread's cleanup object. The string helper receives the address
/// of the string field in ECX, plus its pointer and length on the stack. This
/// thiscall wrapper removes one unused stack word and returns `this`.
lf_checker_rt::export!(thiscall, rw_005c3330(this: u32, unused_arg: u32) -> u32 {
    const SECONDARY_POINTER_OFFSET: u32 = 0x144;
    const SECONDARY_ENABLED_OFFSET: u32 = 0x14A;
    const STRING_POINTER_OFFSET: u32 = 0x14C;
    const STRING_LENGTH_OFFSET: u32 = 0x152;
    const TLS_ENV_POINTER_OFFSET: u32 = 8;
    const CLEANUP_VTABLE_RELEASE_SLOT_OFFSET: u32 = 12;
    const DELETE_STRING_ARRAY_CALLEE: u32 = 1;

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

    let length = unsafe { read_u16(this + STRING_LENGTH_OFFSET) } as u32;
    if length != 0 {
        let string_field = this.wrapping_add(STRING_POINTER_OFFSET);
        let string_array = unsafe { read_u32(string_field) };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            DELETE_STRING_ARRAY_CALLEE,
            u32,
            string_field,
            string_array,
            length
        );
    }

    if unsafe { read_u16(this + SECONDARY_ENABLED_OFFSET) } != 0 {
        let secondary = unsafe { read_u32(this + SECONDARY_POINTER_OFFSET) };
        if secondary != 0 {
            unsafe { release_through_tls(secondary) };
        }
    }

    if this != 0 {
        unsafe { release_through_tls(this) };
    }
    let _ = unused_arg;
    this
});
