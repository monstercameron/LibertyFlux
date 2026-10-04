// original: 0x00c0f850 UILayoutFrame::vf101
/// Store a dword into field `0xe4` of this layout frame.
/// Returns the stored value.
export!(thiscall, rw_00c0f850(this: *mut u8, value: u32) -> u32 {
    unsafe {
        *(this.add(0xe4) as *mut u32) = value;
        value
    }
});
