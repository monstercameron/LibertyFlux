// original: 0x00d8e810 audio_struct_zero16
/// Zero the 16-byte audio scratch struct at `this` and return `this`.
///
/// The original clears four dwords starting at the object pointer; the
/// object holds no vtable or tag, only scalar fields reset together.
lf_rs89_rt::export!(thiscall, rw_00d8e810(this: *mut u8) -> u32 {
    unsafe {
        core::ptr::write_bytes(this, 0, 16);
        this as u32
    }
});
