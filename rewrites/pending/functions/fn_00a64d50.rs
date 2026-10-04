// original: 0x00a64d50 UILayoutFrame::vf89
/// Stores the given value into the layout-frame field at +0xD4.
export!(thiscall, rw_00a64d50(this: u32, value: u32) -> u32 {
    unsafe {
        *((this + 0xD4) as *mut u32) = value;
    }
    0
});
