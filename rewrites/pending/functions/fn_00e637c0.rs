// original: 0x00e637c0 init_pool_tagged_s0db0_c2
/// Initialise 2 records of 0xDB0 bytes starting at 0x11A32C0.
///
/// Each element is passed with three constant arguments to the stdcall/4
/// init routine (stubbed), then gets the tag word at offset 0xD74 stamped
/// with `0xFFFFFFFF`. Returns the last init answer.
export!(cdecl, rw_00e637c0() -> u32 {
    unsafe {
        const BASE: u32 = 0x11A32C0;
        const COUNT: u32 = 2;
        const STRIDE: u32 = 0xDB0;
        const TAG_OFF: u32 = 0xD74;
        let init: extern "stdcall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let func = relocated(0x4065E0);
        let mut ptr = relocated(BASE);
        let mut last = 0u32;
        for _ in 0..COUNT {
            last = init(ptr, 0x10, 0xC8, func);
            *((ptr + TAG_OFF) as *mut u32) = 0xFFFFFFFF;
            ptr = ptr.wrapping_add(STRIDE);
        }
        last
    }
});
