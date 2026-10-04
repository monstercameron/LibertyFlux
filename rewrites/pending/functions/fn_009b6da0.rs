// original: 0x009b6da0 alloc_init_input_obj_E94398
/// Allocate and initialise a 0xc-byte input object (vtable 0xe94398).
///
/// Requests 0xc bytes from the game allocator (cdecl/1, stubbed by the
/// checker) and returns null when it fails. Otherwise stores the vtable
/// pointer at +0, clears the flag byte at +4 and records the dword at +8, and returns the block.
export!(stdcall, rw_009b6da0(value: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xe94398;
        const SIZE: u32 = 0xc;
        let obj = callee_cdecl!(1, u32, SIZE);
        if obj == 0 {
            return 0;
        }
        let base = obj as *mut u8;
        core::ptr::write(base as *mut u32, relocated(VTABLE));
        core::ptr::write(base.add(4), 0u8);
        core::ptr::write(base.add(8) as *mut u32, value);
        obj
    }
});