// original: 0x006f5720 reset_if_pending
/// Reset the state block when its pending flag is set.
///
/// When bit 0 at `+0x88` is set, runs a teardown helper unless the mode
/// word at `+0x20` is already 4, restores the default words, clears the
/// flag and zeroes the tail words. No meaningful return value.
rt::export!(thiscall, rw_006f5720(this: *mut u8) -> u32 {
    unsafe {
        if (*this.add(0x88) & 1) != 0 {
            if *this.add(0x20).cast::<u32>() != 4 {
                rt::callee_thiscall!(1, u32, this as u32);
                *this.add(4).cast::<u32>() = 0xFFFF_FFFF;
                *this.add(8).cast::<u16>() = 0;
                *this.add(0xC).cast::<u32>() = 0xFFFF_FFFF;
                *this.add(0x10).cast::<u16>() = 0;
                *this.add(0x18).cast::<u32>() = 0xFFFF_FFFF;
                *this.add(0x20).cast::<u32>() = 4;
            }
            *this.add(0x88) &= 0xFE;
            *this.add(0x14).cast::<u32>() = 0xFFFF_FFFF;
            *this.add(0x1C).cast::<u32>() = 0;
            *this.add(0x74).cast::<u32>() = 0;
            *this.add(0x78).cast::<u32>() = 0;
        }
        0
    }
});
