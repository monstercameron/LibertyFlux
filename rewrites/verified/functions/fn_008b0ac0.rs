// original: 0x008b0ac0 audio_block_init_defaults
/// Large constant initializer for the audio parameter block.
///
/// Stamps the default gains, flags and zeroed meter slots across the block.
export!(thiscall, rw_008b0ac0(this: u32) -> () {
    unsafe {
        st32(this.wrapping_add(0x172c), 0);
        st32(this.wrapping_add(0x1730), 0x469c4000);
        st32(this.wrapping_add(0x1734), 0x3dcccccd);
        st32(this.wrapping_add(0x1738), 0x3f800000);
        st32(this.wrapping_add(0x173c), 0x40a00000);
        st32(this.wrapping_add(0x1740), 0x3f4ccccd);
        st32(this.wrapping_add(0x1744), 0x3e19999a);
        st32(this.wrapping_add(0x1748), 0);
        st32(this.wrapping_add(0x174c), 0x3f000000);
        st32(this.wrapping_add(0x1728), 0x3f800000);
        st32(this.wrapping_add(0x1724), 0x3f800000);
        ((this.wrapping_add(0x1794)) as *mut u16).write_unaligned(0x0100);
        st32(this.wrapping_add(0x1797), 0x01010101);
        ((this.wrapping_add(0x1796)) as *mut u8).write_unaligned(1);
        st32(this.wrapping_add(0x179b), 0x00010000);
        ((this.wrapping_add(0x17e0)) as *mut u8).write_unaligned(0);
        for off in [0x17a0u32, 0x17a8, 0x17b0, 0x17b8, 0x17c0, 0x17c8] {
            ((this.wrapping_add(off)) as *mut u64).write_unaligned(0);
        }
        for off in [0x1754u32, 0x1764, 0x1774, 0x1784, 0x1758, 0x1768,
                    0x1778, 0x1788, 0x175c, 0x176c, 0x177c, 0x178c,
                    0x1760, 0x1770, 0x1780, 0x1790] {
            st32(this.wrapping_add(off), 0);
        }
        st32(this.wrapping_add(0x17d4), 0x3f800000);
        st32(this.wrapping_add(0x17d0), 0x3f4ccccd);
        st32(this.wrapping_add(0x17dc), 0x3f800000);
        st32(this.wrapping_add(0x1750), 0x3f800000);
        st32(this.wrapping_add(0x17e8), 0xffffffff);
        st32(this.wrapping_add(0x17e4), 0xffffffff);
        ((this.wrapping_add(0x17f4)) as *mut u8).write_unaligned(0);
    }
});
