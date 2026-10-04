// original: 0x00c0f830 UILayoutFrame::vf98
/// Store a dword into field `0xe0` of this layout frame.
/// Returns the stored value.
export!(thiscall, rw_00c0f830(this: *mut u8, value: u32) -> u32 {
    unsafe {
        *(this.add(0xe0) as *mut u32) = value;
        value
    }
});
