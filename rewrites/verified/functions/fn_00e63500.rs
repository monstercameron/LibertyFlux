// original: 0x00e63500 init_pool_s100_c16
/// Initialise 16 records of 0x100 bytes starting at 0x11A0B10.
///
/// Invokes the element init routine (thiscall/0, stubbed by the checker) on
/// each record front to back. Returns the last init answer, matching the
/// value the original leaves in EAX.
export!(cdecl, rw_00e63500() -> u32 {
    unsafe {
        const BASE: u32 = 0x11A0B10;
        const COUNT: u32 = 16;
        const STRIDE: u32 = 0x100;
        let init: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let mut ptr = relocated(BASE);
        let mut last = 0u32;
        for _ in 0..COUNT {
            last = init(ptr);
            ptr = ptr.wrapping_add(STRIDE);
        }
        last
    }
});
