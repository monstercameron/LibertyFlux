// original: 0x009b6ea0 NativeImpl_ENABLE_DEBUG_CAM
/// Allocate and initialise a 0xc-byte input object (vtable 0xe944a8).
///
/// Requests 0xc bytes from the game allocator (cdecl/1, stubbed by the
/// checker) and returns null when it fails. Otherwise stores the vtable
/// pointer at +0, clears the flag byte at +4 and records the flag byte at +8, and returns the block.
export!(stdcall, rw_009b6ea0(flag: u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xe944a8;
        const SIZE: u32 = 0xc;
        let obj = callee_cdecl!(1, u32, SIZE);
        if obj == 0 {
            return 0;
        }
        let base = obj as *mut u8;
        core::ptr::write(base as *mut u32, relocated(VTABLE));
        core::ptr::write(base.add(4), 0u8);
        core::ptr::write(base.add(8), flag);
        obj
    }
});