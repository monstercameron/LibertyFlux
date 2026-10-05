// original: 0x00a49730 vehicle_classify_flag_bits
/// Classify bits 24..27 of a status word reached through two pointers.
///
/// The word is `[this+0xDC8]+0xEC`, masked with `0x0F000000` (thiscall, no
/// stack arguments). Nibble 0 gives 1, 1 gives 2, 2 gives 3, 4 gives 5 and
/// 8 gives 4; any other nibble gives 0. The 4/8 swap is the original's.
export!(thiscall, rw_00a49730(this: u32) -> u32 {
    unsafe {
        const LINK_OFF: u32 = 0xdc8;
        const WORD_OFF: u32 = 0xec;
        const MASK: u32 = 0x0f00_0000;
        let p = (this.wrapping_add(LINK_OFF) as *const u32).read_unaligned();
        let v = (p.wrapping_add(WORD_OFF) as *const u32).read_unaligned() & MASK;
        match v {
            0x0000_0000 => 1,
            0x0100_0000 => 2,
            0x0200_0000 => 3,
            0x0400_0000 => 5,
            0x0800_0000 => 4,
            _ => 0,
        }
    }
});
