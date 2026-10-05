// original: 0x008F9890 stream_lane_active_check
/// Test whether streaming lane `idx` is active.
///
/// Returns 0 unless `idx` is 0 or 1; otherwise returns whether the
/// 16-bit flag at lane base `+0x9c6` (lanes are 0x4b8 bytes apart) is
/// nonzero. Thiscall, one stack argument, boolean in al.
export!(thiscall, rw_008f9890(this: u32, idx: u32) -> u32 {
    unsafe {
        const LANE_STRIDE: u32 = 0x4b8;
        const ACTIVE_FLAG: u32 = 0x9c6;
        if idx > 1 {
            return 0;
        }
        let w = ((this + idx.wrapping_mul(LANE_STRIDE) + ACTIVE_FLAG) as *const u16)
            .read_unaligned();
        (w != 0) as u32
    }
});
