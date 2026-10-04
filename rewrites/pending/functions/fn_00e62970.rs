// original: 0x00e62970 audio_mark11_slots_used
/// Mark eleven audio slots as used with all-ones sentinel words.
///
/// Each of the eleven 0x30-byte records gets two 0xFFFF_FFFF words at its
/// head; the rest of each record is untouched.
export!(cdecl, rw_00e62970() -> u32 {
    unsafe {
        const BASE: u32 = 0x0116_1540;
        const COUNT: u32 = 11;
        const STRIDE_WORDS: usize = 0x30 / 4;
        const USED: u32 = 0xFFFF_FFFF;
        let mut head = global::<u32>(BASE);
        let mut remaining = COUNT;
        loop {
            *head = USED;
            *head.add(1) = USED;
            head = head.add(STRIDE_WORDS);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        0
    }
});
