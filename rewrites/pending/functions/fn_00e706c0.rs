// original: 0x00e706c0 teardown_100x31f0
/// Run the teardown callee over 100 records of 0x31F0 bytes ending at 0x01B432B8.
///
/// Walks the array from the last element down to the first, invoking the
/// teardown callee (thiscall/0) on each. Returns the last answer, matching
/// the value the original leaves in EAX.
export!(cdecl, rw_00e706c0() -> u32 {
    unsafe {
        const END: u32 = 0x01B432B8;
        const COUNT: u32 = 100;
        const STRIDE: u32 = 0x31F0;
        let mut ptr = relocated(END);
        let mut last = 0u32;
        let mut remaining = COUNT;
        loop {
            ptr = ptr.wrapping_sub(STRIDE);
            last = lf_checker_rt::callee_thiscall!(1, u32, ptr);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        last
    }
});
