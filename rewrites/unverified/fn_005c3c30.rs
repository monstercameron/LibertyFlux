// original: 0x005C3C30 ped_factory_attach_resource

/// Runs a three-argument collection update, requests a result buffer, and
/// attaches it to the third incoming object. When allocation succeeds, the
/// buffer receives a relocated vtable pointer and the identifier at
/// `container + 0x21c`, then the container stores the buffer at +0x22c. A
/// failed allocation clears that container field. The stdcall method returns
/// the third incoming pointer in EAX.
lf_checker_rt::export!(stdcall, rw_005c3c30(_arg0: u32, _arg1: u32, container: u32) -> u32 {
    unsafe {
        const SUBOBJECT_POINTER: u32 = 0x21c;
        const SUBOBJECT_IDENTIFIER: u32 = 0x12c;
        const BUFFER_VTABLE_FILE_VA: u32 = 0x00fe0a84;
        const BUFFER_IDENTIFIER: u32 = 0x04;
        const CONTAINER_BUFFER: u32 = 0x22c;
        const UPDATE_CALLEE: u32 = 1;
        const ALLOCATE_BUFFER_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let _: u32 = lf_checker_rt::callee_stdcall!(
            UPDATE_CALLEE,
            u32,
            container,
            _arg0,
            _arg1,
        );
        let allocate_buffer: extern "cdecl" fn() -> u32 =
            core::mem::transmute(lf_checker_rt::callee_addr(ALLOCATE_BUFFER_CALLEE) as usize);
        let buffer = allocate_buffer();
        if buffer == 0 {
            write_word(container + CONTAINER_BUFFER, 0);
        } else {
            let subobject = read_word(container + SUBOBJECT_POINTER);
            let identifier = read_word(subobject + SUBOBJECT_IDENTIFIER);
            write_word(buffer + BUFFER_IDENTIFIER, identifier);
            write_word(buffer, lf_checker_rt::relocated(BUFFER_VTABLE_FILE_VA));
            write_word(container + CONTAINER_BUFFER, buffer);
        }
        container
    }
});
