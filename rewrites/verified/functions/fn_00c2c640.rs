// original: 0x00c2c640 audEntityRadioEmitter::vf6
/// Null-or-flag check.
///
/// Returns true for a null inner object, otherwise true exactly when the
/// inner object selects the radio path.
export!(thiscall, rw_00c2c640(this: *const u8) -> u32 {
    unsafe {
        let inner = *(this.add(4) as *const u32);
        if inner == 0 {
            return 1;
        }
        (((*(inner.wrapping_add(0x28) as *const u32)) & 0x3C0) == 0x80) as u32
    }
});
