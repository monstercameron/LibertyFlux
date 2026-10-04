// original: 0x00e63730 init_pool_s040_c16
/// Initialise 16 records of 0x40 bytes starting at 0x11A2A70.
///
/// Same shape as [`rw_00e63500`]: front-to-back init sweep returning the
/// last init answer.
export!(cdecl, rw_00e63730() -> u32 {
    unsafe {
        const BASE: u32 = 0x11A2A70;
        const COUNT: u32 = 16;
        const STRIDE: u32 = 0x40;
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
