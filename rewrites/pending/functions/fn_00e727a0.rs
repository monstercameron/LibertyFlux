// original: 0x00e727a0 teardown_array_6x50
/// Teardown loop over 6 records of 0x50 bytes ending at 0x169EA50.
///
/// Same shape as [`rw_00e726c0`]: back-to-front destructor sweep, returning
/// the last destructor answer.
export!(cdecl, rw_00e727a0() -> u32 {
    unsafe {
        const END: u32 = 0x169EA50;
        const COUNT: u32 = 6;
        const STRIDE: u32 = 0x50;
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
