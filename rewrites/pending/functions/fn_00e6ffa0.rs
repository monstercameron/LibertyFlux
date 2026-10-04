// original: 0x00E6FFA0 timing_block_reset_c
/// Reset timing control block C to its idle state.
///
/// Same layout as blocks A and B: clears the run flag bit, zeroes the six
/// counter words and restores the default limit of 100.
export!(cdecl, rw_00e6ffa0() -> u32 {
    unsafe {
        const BASE: u32 = 0x019FE68C;
        const FLAG: u32 = 0x019FE6AC;
        const DEFAULT_LIMIT: u32 = 100;
        *global::<u8>(FLAG) &= 0xFE;
        *global::<u32>(BASE) = 0;
        *global::<u32>(BASE + 0x04) = 0;
        *global::<u32>(BASE + 0x0C) = 0;
        *global::<u32>(BASE + 0x08) = 0;
        *global::<u32>(BASE + 0x10) = 0;
        *global::<u32>(BASE + 0x18) = 0;
        *global::<u32>(BASE + 0x1C) = DEFAULT_LIMIT;
        0
    }
});
