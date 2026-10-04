// original: 0x00b79320 CTaskMoveInterface::vf1
/// Clamp a float into the object and set flag bit 0 from a byte.
///
/// Clamps the float argument into the range 0.0 to 3.0 (NaN passes
/// through unchanged) and stores it at offset `0x4`; sets bit 0 of the
/// word at offset `0x8` to bit 0 of the flags argument. Returns that bit.
export!(thiscall, rw_00b79320(this: u32, x: f32, flags: u32) -> u32 {
    let c = if x < 0.0 {
        0.0
    } else if x > 3.0 {
        3.0
    } else {
        x
    };
    unsafe {
        let w = *((this + 8) as *const u32);
        *((this + 4) as *mut f32) = c;
        let e = ((flags as u8 as u32) ^ w) & 1;
        *((this + 8) as *mut u32) = w ^ e;
        e
    }
});
