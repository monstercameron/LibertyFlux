// original: 0x0066cdb0 rage::rmPtfxShaderVar_Float2::create

/// Create a particle-effect shader variable through allocator slot `+8`
/// in the object held at TLS slot zero. The indirect allocator receives
/// a request for 48 bytes, alignment 16, and flags zero.
/// A null allocator result is returned unchanged. On success, install the
/// relocated class descriptor and initialize the fields listed below.
/// The factory has no input parameters and returns the allocated object.
lf_checker_rt::export!(cdecl, rw_0066cdb0() -> u32 {
    unsafe {
        const CLASS_INFO_FILE_VA: u32 = 0x00fe2cd4;
        const ALLOCATION_SIZE: u32 = 0x30;
        const ALLOCATOR_ALIGNMENT: u32 = 0x10;
        const ALLOCATOR_FLAGS: u32 = 0;
        const TLS_ALLOCATOR_OFFSET: u32 = 8;
        const VTABLE_METHOD_OFFSET: u32 = 8;

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
        let allocation_entry = read_word(vtable + VTABLE_METHOD_OFFSET);
        let allocate: unsafe extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocation_entry as usize);
        let object = allocate(allocator_object, ALLOCATION_SIZE, ALLOCATOR_ALIGNMENT, ALLOCATOR_FLAGS);
        if object == 0 {
            return 0;
        }

        write_word(object, lf_checker_rt::relocated(CLASS_INFO_FILE_VA));
        write_word(object + 0x8, 0x0);
        write_word(object + 0x4, 0x0);
        (object.wrapping_add(0x10) as *mut u16).write_unaligned(0x100);
        write_word(object + 0x20, 0x0);
        write_word(object + 0x24, 0x0);
        (object.wrapping_add(0x12) as *mut u8).write(0x3);
        object
    }
});
