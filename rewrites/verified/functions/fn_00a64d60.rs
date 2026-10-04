// original: 0x00a64d60 field_e4_float_store
/// Stores the given float bits into the object field at +0xE4.
export!(thiscall, rw_00a64d60(this: u32, fbits: u32) -> u32 {
    unsafe {
        *((this + 0xE4) as *mut u32) = fbits;
    }
    0
});
