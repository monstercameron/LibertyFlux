// original: 0x00e727c0 teardown_array_16x1d0
/// Teardown loop over 16 records of 0x1d0 bytes ending at 0x16c3cd0.
///
/// Walks the array from the last element down to the first, invoking the
/// element destructor (thiscall/0, stubbed by the checker) on each. Returns
/// the last destructor answer, matching the value the original leaves in EAX.
export!(cdecl, rw_00e727c0() -> u32 {
    unsafe {
        const END: u32 = 0x16c3cd0;
        const COUNT: u32 = 16;
        const STRIDE: u32 = 0x1d0;
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
