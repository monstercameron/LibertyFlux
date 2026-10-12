// original: 0x0066cf10 rage::rmPtfxShaderVar_Keyframe::create

/// Allocate a keyframe shader variable through slot `+8` on the allocator in
/// TLS slot zero, requesting 80 bytes with 16-byte alignment and zero flags.
/// Return null when allocation fails. On success, call the keyframe
/// constructor with the allocated address as its this pointer and return the
/// constructor's result. The factory is parameterless cdecl; its outgoing
/// allocator and constructor calls remain observable through checker stubs.
lf_checker_rt::export!(cdecl, rw_0066cf10() -> u32 {
    unsafe {
        const ALLOCATION_SIZE: u32 = 0x50;
        const ALLOCATOR_ALIGNMENT: u32 = 0x10;
        const ALLOCATOR_FLAGS: u32 = 0;
        const TLS_ALLOCATOR_OFFSET: u32 = 8;
        const VTABLE_ALLOCATE: u32 = 8;
        const KEYFRAME_CONSTRUCTOR_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let tls_value = lf_checker_rt::tls_slot(0);
        let allocator_object = read_word(tls_value + TLS_ALLOCATOR_OFFSET);
        let vtable = read_word(allocator_object);
        let allocate_entry = read_word(vtable + VTABLE_ALLOCATE);
        let allocate: unsafe extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocate_entry as usize);
        let object = allocate(
            allocator_object,
            ALLOCATION_SIZE,
            ALLOCATOR_ALIGNMENT,
            ALLOCATOR_FLAGS,
        );
        if object == 0 {
            return 0;
        }

        lf_checker_rt::callee_thiscall!(KEYFRAME_CONSTRUCTOR_CALLEE, u32, object)
    }
});
