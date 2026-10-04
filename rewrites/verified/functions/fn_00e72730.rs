// original: 0x00e72730 teardown_array_71x48
/// Teardown loop over 71 records of 0x48 bytes ending at 0x166FED8.
///
/// Same shape as [`rw_00e726c0`]: back-to-front destructor sweep, returning
/// the last destructor answer.
export!(cdecl, rw_00e72730() -> u32 {
    unsafe {
        const END: u32 = 0x166FED8;
        const COUNT: u32 = 71;
        const STRIDE: u32 = 0x48;
        let destroy: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let mut ptr = relocated(END);
        let mut last = 0u32;
        let mut remaining = COUNT;
        loop {
            ptr = ptr.wrapping_sub(STRIDE);
            last = destroy(ptr);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        last
    }
});
