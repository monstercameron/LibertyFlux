// original: 0x0097AC10 audio_vec_zero16
/// Clear the 16-byte vector at the object base.
///
/// Returns nothing meaningful (exit eax is entry garbage, unchecked).
/// thiscall(obj).
export!(thiscall, rw_s103_97ac10(obj: *mut u8) -> u32 {
    unsafe {
        core::ptr::write_bytes(obj, 0, 16);
        0
    }
});
