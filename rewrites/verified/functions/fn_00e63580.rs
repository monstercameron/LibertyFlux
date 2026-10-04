// original: 0x00e63580 init_pool_s100_c32
/// Initialise 32 records of 0x100 bytes starting at 0x119D0F0.
///
/// Same shape as [`rw_00e63500`]: front-to-back init sweep returning the
/// last init answer.
export!(cdecl, rw_00e63580() -> u32 {
    unsafe {
        const BASE: u32 = 0x119D0F0;
        const COUNT: u32 = 32;
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
