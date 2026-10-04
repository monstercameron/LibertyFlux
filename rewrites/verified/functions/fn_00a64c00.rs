// original: 0x00a64c00 field_e0_float_store
/// Stores the given float bits into the object field at +0xE0.
export!(thiscall, rw_00a64c00(this: u32, fbits: u32) -> u32 {
    unsafe {
        *((this + 0xE0) as *mut u32) = fbits;
    }
    0
});
