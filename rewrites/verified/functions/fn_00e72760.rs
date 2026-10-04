// original: 0x00e72760 teardown_array_128x6c
/// Teardown loop over 128 records of 0x6c bytes ending at 0x1682D80.
///
/// Same shape as [`rw_00e726c0`]: back-to-front destructor sweep, returning
/// the last destructor answer.
export!(cdecl, rw_00e72760() -> u32 {
    unsafe {
        const END: u32 = 0x1682D80;
        const COUNT: u32 = 128;
        const STRIDE: u32 = 0x6c;
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
