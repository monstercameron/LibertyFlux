// original: 0x00c0f5a0 UILayoutFrame::vf79
/// Store a flag byte into field `0xc7` of this layout frame.
///
/// Only the low byte of the argument is kept. Returns the stored byte.
export!(thiscall, rw_00c0f5a0(this: *mut u8, value: u32) -> u32 {
    unsafe {
        *this.add(0xc7) = value as u8;
        value & 0xff
    }
});
