// original: 0x00e729b0 teardown_array_2xd0
/// Teardown loop over 2 records of 0xd0 bytes ending at 0x16C8880.
///
/// Same shape as [`rw_00e726c0`]: back-to-front destructor sweep, returning
/// the last destructor answer.
export!(cdecl, rw_00e729b0() -> u32 {
    unsafe {
        const END: u32 = 0x16C8880;
        const COUNT: u32 = 2;
        const STRIDE: u32 = 0xD0;
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
