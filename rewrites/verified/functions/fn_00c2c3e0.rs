// original: 0x00c2c3e0 audStaticRadioEmitter::vf10
/// Flag-bit test through the handle at +0x30.
///
/// Returns false when the handle is null, otherwise true exactly when
/// bits 2-3 of the tag byte equal binary 01.
export!(thiscall, rw_00c2c3e0(this: *const u8) -> u32 {
    unsafe {
        let inner = *(this.add(0x30) as *const u32);
        if inner == 0 {
            return 0;
        }
        let tag = *((inner + 5) as *const u8);
        ((tag & 0x0C) == 0x04) as u32
    }
});
