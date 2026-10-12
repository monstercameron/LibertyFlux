// original: 0x006672d0 create_keyframe_defaults

/// Allocate the default keyframe record through allocator slot `+8` on the
/// object in TLS slot zero. The request is 68 bytes with 16-byte alignment
/// and zero flags. Return null on allocation failure. Otherwise set the four
/// sentinel words at offsets `+0x14`, `+0x18`, `+0x1c` and `+0x20` to
/// `u32::MAX`, call the default-keyframe initializer with the allocation as
/// its this pointer, and return that call's result. The function is
/// parameterless cdecl; both outgoing calls are compared through checker
/// stubs.
lf_checker_rt::export!(cdecl, rw_006672d0() -> u32 {
    unsafe {
        const ALLOCATION_SIZE: u32 = 0x44;
        const ALLOCATOR_ALIGNMENT: u32 = 0x10;
        const ALLOCATOR_FLAGS: u32 = 0;
        const TLS_ALLOCATOR_OFFSET: u32 = 8;
        const VTABLE_ALLOCATE: u32 = 8;
        const INITIALIZED_WORDS: [u32; 4] = [0x14, 0x18, 0x1c, 0x20];
        const SENTINEL: u32 = u32::MAX;
        const KEYFRAME_INITIALIZER_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
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

        for field_offset in INITIALIZED_WORDS {
            write_word(object + field_offset, SENTINEL);
        }
        lf_checker_rt::callee_thiscall!(KEYFRAME_INITIALIZER_CALLEE, u32, object)
    }
});
