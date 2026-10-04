// original: 0x00c0f8c0 UILayoutFrame::vf22
/// Store a dword into field `0xa0` of this layout frame.
/// Returns the stored value.
export!(thiscall, rw_00c0f8c0(this: *mut u8, value: u32) -> u32 {
    unsafe {
        *(this.add(0xa0) as *mut u32) = value;
        value
    }
});
