// original: 0x00c2c430 audEntityRadioEmitter::vf15
/// Guarded field load (+0xD00), else 0.
///
/// Returns the field only when the inner object exists and selects the
/// radio path; otherwise returns 0.
export!(thiscall, rw_00c2c430(this: *const u8) -> u32 {
    unsafe {
        let inner = *(this.add(4) as *const u32);
        if inner == 0 {
            return 0;
        }
        if ((*(inner.wrapping_add(0x28) as *const u32)) & 0x3C0) == 0x80 {
            *(inner.wrapping_add(0xD00) as *const u32)
        } else {
            0
        }
    }
});
