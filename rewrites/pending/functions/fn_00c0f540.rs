// original: 0x00c0f540 UILayoutFrame::vf40
/// Store a float into field `0xb0` and clear the two tag words at
/// fields `0xb4` and `0xb8` of this layout frame.
export!(thiscall, rw_00c0f540(this: *mut u8, value: f32) -> u32 {
    unsafe {
        *(this.add(0xb0) as *mut f32) = value;
        *(this.add(0xb4) as *mut u32) = 0;
        *(this.add(0xb8) as *mut u32) = 0;
        0
    }
});
