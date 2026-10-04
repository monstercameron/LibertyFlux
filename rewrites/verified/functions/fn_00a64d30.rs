// original: 0x00a64d30 UILayoutFrame::vf95
/// Stores the given value into the layout-frame field at +0xDC.
export!(thiscall, rw_00a64d30(this: u32, value: u32) -> u32 {
    unsafe {
        *((this + 0xDC) as *mut u32) = value;
    }
    0
});
