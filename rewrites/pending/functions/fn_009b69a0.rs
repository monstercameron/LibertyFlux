// original: 0x009b69a0 alloc_init_input_obj_E94338
/// Allocate and initialise a 0x10-byte input object (vtable 0xe94338).
///
/// Requests 0x10 bytes from the game allocator (cdecl/1, stubbed by the
/// checker) and returns null when it fails. Otherwise stores the vtable
/// pointer at +0, clears the flag byte at +4 and records the two dwords at +8 and +0xc, and returns the block.
export!(stdcall, rw_009b69a0(first: u32, second: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xe94338;
        const SIZE: u32 = 0x10;
        let obj = callee_cdecl!(1, u32, SIZE);
        if obj == 0 {
            return 0;
        }
        let base = obj as *mut u8;
        core::ptr::write(base as *mut u32, relocated(VTABLE));
        core::ptr::write(base.add(4), 0u8);
        core::ptr::write(base.add(8) as *mut u32, first);
        core::ptr::write(base.add(0x0c) as *mut u32, second);
        obj
    }
});