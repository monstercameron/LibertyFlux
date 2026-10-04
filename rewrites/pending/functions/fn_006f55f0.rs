// original: 0x006f55f0 init_state_block
/// Initialise a large state block with default constants.
///
/// Installs the vtable, fills sentinel words (`-1`), zero words, timing
/// constants and a flag byte, and masks the mode bits at `+0x88`.
/// Returns `this`.
rt::export!(thiscall, rw_006f55f0(this: *mut u8) -> u32 {
    unsafe {
        *this.cast::<u32>() = rt::relocated(0x00FE543C);
        *this.add(4).cast::<u32>() = 0xFFFF_FFFF;
        *this.add(0xC).cast::<u32>() = 0xFFFF_FFFF;
        *this.add(8).cast::<u16>() = 0;
        *this.add(0x10).cast::<u16>() = 0;
        for off in [0x1Cusize, 0x24, 0x28, 0x2C] {
            *this.add(off).cast::<u32>() = 0;
        }
        *this.add(0x14).cast::<u32>() = 0xFFFF_FFFF;
        *this.add(0x18).cast::<u32>() = 0xFFFF_FFFF;
        *this.add(0x20).cast::<u32>() = 4;
        *this.add(0x30).cast::<u32>() = 0xFA;
        *this.add(0x38).cast::<u32>() = 0x3E8;
        *this.add(0x40) = 1;
        *this.add(0x3C).cast::<u32>() = 0x1F40;
        *this.add(0x34).cast::<u32>() = 0x20;
        for off in [0x44usize, 0x48, 0x4C, 0x50, 0x54, 0x58, 0x5C, 0x60, 0x68, 0x6C] {
            *this.add(off).cast::<u32>() = 0;
        }
        *this.add(0x88) &= 0xC8;
        for off in [0x74usize, 0x78, 0x7C, 0x80] {
            *this.add(off).cast::<u32>() = 0;
        }
        this as u32
    }
});
