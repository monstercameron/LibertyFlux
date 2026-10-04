// original: 0x009b6940 alloc_init_input_obj_E942A8
/// Allocate and initialise a 0x10-byte input object (vtable 0xe942a8).
///
/// Requests 0x10 bytes from the game allocator (cdecl/1, stubbed by the
/// checker) and returns null when it fails. Otherwise stores the vtable
/// pointer at +0, clears the flag byte at +4 and records the dword at +8 and the flag byte at +0xc, and returns the block.
export!(stdcall, rw_009b6940(value: u32, flag: u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xe942a8;
        const SIZE: u32 = 0x10;
        let obj = callee_cdecl!(1, u32, SIZE);
        if obj == 0 {
            return 0;
        }
        let base = obj as *mut u8;
        core::ptr::write(base as *mut u32, relocated(VTABLE));
        core::ptr::write(base.add(4), 0u8);
        core::ptr::write(base.add(8) as *mut u32, value);
        core::ptr::write(base.add(0x0c), flag);
        obj
    }
});