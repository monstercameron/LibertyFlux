// original: 0x008F98C0 stream_lane_flag_check
/// Test the flag byte of streaming lane `idx`.
///
/// Returns 0 unless `idx` is 0 or 1; otherwise returns whether the
/// byte at lane base `+0xe7c` (lanes are 0x4b8 bytes apart) is
/// nonzero. Thiscall, one stack argument, boolean in al.
export!(thiscall, rw_008f98c0(this: u32, idx: u32) -> u32 {
    unsafe {
        const LANE_STRIDE: u32 = 0x4b8;
        const FLAG_BYTE: u32 = 0xe7c;
        if idx > 1 {
            return 0;
        }
        let b = ((this + idx.wrapping_mul(LANE_STRIDE) + FLAG_BYTE) as *const u8).read();
        (b != 0) as u32
    }
});
