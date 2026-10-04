// original: 0x00e62900 audio_fill12_quad_pattern
/// Fill twelve 16-byte audio blocks with a mirrored constant pattern.
///
/// Each block holds the pair (HI, LO, LO, HI): the outer words share one
/// constant and the inner words another. Twelve blocks are written back to
/// back starting at the fill base.
export!(cdecl, rw_00e62900() -> u32 {
    unsafe {
        const BASE: u32 = 0x0116_1740;
        const COUNT: u32 = 12;
        const STRIDE: u32 = 0x10;
        const HI: u32 = 0x4974_2400;
        const LO: u32 = 0xC974_2400;
        let mut block = global::<u32>(BASE);
        let mut remaining = COUNT;
        loop {
            *block = HI;
            *block.add(1) = LO;
            *block.add(2) = LO;
            *block.add(3) = HI;
            block = block.add(STRIDE as usize / 4);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        0
    }
});
