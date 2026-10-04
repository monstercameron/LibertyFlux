// original: 0x00e72a70 teardown_array_117x30
/// Teardown loop over 117 records of 0x30 bytes ending at 0x16D37B0.
///
/// Same shape as [`rw_00e726c0`]: back-to-front destructor sweep, returning
/// the last destructor answer.
export!(cdecl, rw_00e72a70() -> u32 {
    unsafe {
        const END: u32 = 0x16D37B0;
        const COUNT: u32 = 117;
        const STRIDE: u32 = 0x30;
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
