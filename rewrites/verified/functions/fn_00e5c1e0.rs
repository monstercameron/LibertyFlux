// original: 0x00e5c1e0 zero_table_d568_and_register
/// Zeroes dwords +0x0 and +0x8 of 64 sixteen-byte rows at 0x018dd568,
/// clears the trailing word at 0x018dd968, then registers tag 0x00e6e330.
/// Returns the registrar's answer.
export!(cdecl, rw_00e5c1e0() -> u32 {
    unsafe {
        const BASE: u32 = 0x018DD568;
        const ROWS: u32 = 64;
        const STRIDE: u32 = 16;
        const TRAILER: u32 = 0x018DD968;
        const TAG: u32 = 0x00E6E330;
        let base = relocated(BASE);
        let mut i = 0u32;
        while i < ROWS {
            let row = base.wrapping_add(i.wrapping_mul(STRIDE));
            *(row as *mut u32) = 0;
            *(row.wrapping_add(8) as *mut u32) = 0;
            i = i.wrapping_add(1);
        }
        *global::<u32>(TRAILER) = 0;
        callee_cdecl!(1, u32, relocated(TAG))
    }
});
