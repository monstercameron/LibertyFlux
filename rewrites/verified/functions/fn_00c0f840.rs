// original: 0x00c0f840 UILayoutFrame::vf17
/// Store a flag byte into field `0xeb` of this layout frame.
///
/// Only the low byte of the argument is kept. Returns the stored byte.
export!(thiscall, rw_00c0f840(this: *mut u8, value: u32) -> u32 {
    unsafe {
        *this.add(0xeb) = value as u8;
        value & 0xff
    }
});
