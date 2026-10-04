// original: 0x00e70790 delete_critsec_array_64x2c
/// Release the 64-entry critical-section array ending at 0x01B48FE0.
///
/// Walks 64 records of 0x2C bytes from the top down, passing each element
/// to the imported `DeleteCriticalSection`. Returns the last answer,
/// matching the value the original leaves in EAX.
export!(cdecl, rw_00e70790() -> u32 {
    unsafe {
        const END: u32 = 0x01B48FE0;
        const COUNT: u32 = 64;
        const STRIDE: u32 = 0x2C;
        let mut ptr = relocated(END);
        let mut last = 0u32;
        let mut remaining = COUNT;
        loop {
            ptr = ptr.wrapping_sub(STRIDE);
            last = lf_checker_rt::callee_stdcall!(1, u32, ptr);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        last
    }
});
