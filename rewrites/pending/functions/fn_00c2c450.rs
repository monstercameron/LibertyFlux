// original: 0x00c2c450 audEntityRadioEmitter::vf14
/// Guarded field load (+0xCFC), else 0x5D5C.
///
/// Returns the field only when the inner object exists and selects the
/// radio path; otherwise returns the default station id 0x5D5C.
export!(thiscall, rw_00c2c450(this: *const u8) -> u32 {
    unsafe {
        let inner = *(this.add(4) as *const u32);
        if inner == 0 {
            return 0x5D5C;
        }
        if ((*(inner.wrapping_add(0x28) as *const u32)) & 0x3C0) == 0x80 {
            *(inner.wrapping_add(0xCFC) as *const u32)
        } else {
            0x5D5C
        }
    }
});
