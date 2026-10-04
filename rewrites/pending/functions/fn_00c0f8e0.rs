// original: 0x00c0f8e0 UIFrame::vf115
/// Store a dword into field `0x1dc` of this frame.
/// Returns the stored value.
export!(thiscall, rw_00c0f8e0(this: *mut u8, value: u32) -> u32 {
    unsafe {
        *(this.add(0x1dc) as *mut u32) = value;
        value
    }
});
