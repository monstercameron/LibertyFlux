// original: 0x00c0f510 UILayoutFrame::vf49
/// Store a float into field `0x8` of this layout frame.
export!(thiscall, rw_00c0f510(this: *mut u8, value: f32) -> u32 {
    unsafe {
        *(this.add(0x8) as *mut f32) = value;
        0
    }
});
