// original: 0x00c0f500 UILayoutFrame::vf70
/// Store a flag byte into field `0xc4` of this layout frame.
///
/// Only the low byte of the argument is kept. Returns the stored byte.
export!(thiscall, rw_00c0f500(this: *mut u8, value: u32) -> u32 {
    unsafe {
        *this.add(0xc4) = value as u8;
        value & 0xff
    }
});
