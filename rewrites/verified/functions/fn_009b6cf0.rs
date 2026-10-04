// original: 0x009b6cf0 alloc_input_obj_E94168
/// Allocate and initialise a 0x8-byte input object (vtable 0xe94168).
///
/// Requests 0x8 bytes from the game allocator (cdecl/1, stubbed by the
/// checker) and returns null when it fails. Otherwise stores the vtable
/// pointer at +0, clears the flag byte at +4, and returns the block.
export!(stdcall, rw_009b6cf0() -> u32 {
    unsafe {
        const VTABLE: u32 = 0xe94168;
        const SIZE: u32 = 0x8;
        let obj = callee_cdecl!(1, u32, SIZE);
        if obj == 0 {
            return 0;
        }
        let base = obj as *mut u8;
        core::ptr::write(base as *mut u32, relocated(VTABLE));
        core::ptr::write(base.add(4), 0u8);
        obj
    }
});