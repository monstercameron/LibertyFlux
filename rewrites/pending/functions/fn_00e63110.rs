// original: 0x00e63110 init_strided_slots_minus_one_64
/// Write -1 into 64 dword slots spaced 8 bytes apart (every other dword of a
/// 512-byte span) and return the span's end address, which is what the
/// original leaves in EAX.
export!(cdecl, rw_00e63110() -> u32 {
    const SLOTS: u32 = 64;
    const STRIDE_WORDS: u32 = 2;
    const FILL: u32 = 0xFFFF_FFFF;
    unsafe {
        let base = global::<u32>(0x01179690);
        let mut i = 0;
        while i < SLOTS {
            *base.add((i * STRIDE_WORDS) as usize) = FILL;
            i += 1;
        }
        relocated(0x01179690).wrapping_add(SLOTS * STRIDE_WORDS * 4)
    }
});
