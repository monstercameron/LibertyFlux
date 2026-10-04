// original: 0x00c0f860 UILayoutFrame::vf92
/// Store a dword into field `0xd8` of this layout frame.
/// Returns the stored value.
export!(thiscall, rw_00c0f860(this: *mut u8, value: u32) -> u32 {
    unsafe {
        *(this.add(0xd8) as *mut u32) = value;
        value
    }
});
