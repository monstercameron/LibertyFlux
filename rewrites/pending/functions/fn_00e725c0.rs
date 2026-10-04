// original: 0x00e725c0 reinit_array_40x6c
/// Teardown loop over 40 records of 0x6c bytes ending at 0x1661444.
///
/// Walks the array from the last element down to the first, invoking the
/// element destructor (thiscall/0, stubbed by the checker) on each. Returns
/// the last destructor answer, matching the value the original leaves in EAX.
/// Stamps the tag word at the head of each element before invoking its
/// destructor, matching the original's store-then-call order.
export!(cdecl, rw_00e725c0() -> u32 {
    unsafe {
        const END: u32 = 0x1661444;
        const COUNT: u32 = 40;
        const STRIDE: u32 = 0x6c;
        const TAG: u32 = 0xe83134;
        let destroy: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let mut ptr = relocated(END);
        let mut last = 0u32;
        let mut remaining = COUNT;
        loop {
            ptr = ptr.wrapping_sub(STRIDE);
            *(ptr as *mut u32) = relocated(TAG);
            last = destroy(ptr);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        last
    }
});
