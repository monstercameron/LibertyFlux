// original: 0x00e615a0 timing_records_clear_three
/// Invalidate three timing records and return zero.
///
/// Each 8-byte record gets an invalid marker word and a cleared flag word.
export!(cdecl, rw_00e615a0() -> u32 {
    unsafe {
        const BASE: u32 = 0x01A021FC;
        const RECORDS: u32 = 3;
        const STRIDE: u32 = 8;
        for i in 0..RECORDS {
            *global::<u32>(BASE + i * STRIDE) = 0xFFFF_FFFF;
            *global::<u16>(BASE + i * STRIDE + 4) = 0;
        }
        0
    }
});
