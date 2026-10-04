// original: 0x00e63130 init_array_32x20
/// Forward init sweep over 32 records of 0x20 bytes at 0x1177680.
///
/// Invokes the element routine (thiscall/0, stubbed by the checker) on each
/// record in ascending address order. Returns the last answer, matching the
/// value the original leaves in EAX.
export!(cdecl, rw_00e63130() -> u32 {
    unsafe {
        const BASE: u32 = 0x1177680;
        const COUNT: u32 = 32;
        const STRIDE: u32 = 0x20;
        let init: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let mut ptr = relocated(BASE);
        let mut last = 0u32;
        let mut remaining = COUNT;
        loop {
            last = init(ptr);
            ptr = ptr.wrapping_add(STRIDE);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        last
    }
});
