// original: 0x00c0f520 UILayoutFrame::vf47
/// Store a float into field `0x4` of this layout frame.
export!(thiscall, rw_00c0f520(this: *mut u8, value: f32) -> u32 {
    unsafe {
        *(this.add(0x4) as *mut f32) = value;
        0
    }
});
