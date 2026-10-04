// original: 0x00e61630 timing_record_clear_one
/// Invalidate one timing record and return zero.
///
/// Writes an invalid marker word and a cleared flag word.
export!(cdecl, rw_00e61630() -> u32 {
    unsafe {
        *global::<u32>(0x01A02244) = 0xFFFF_FFFF;
        *global::<u16>(0x01A02248) = 0;
        0
    }
});
