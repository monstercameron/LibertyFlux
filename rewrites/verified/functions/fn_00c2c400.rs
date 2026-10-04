// original: 0x00c2c400 audEntityRadioEmitter::vf2
/// Guarded tail call, falling back to a global default float.
///
/// When the inner object exists and selects the radio path, forwards to
/// the shared routine with the inner object plus 0x210; otherwise returns
/// the global default as a float on ST0.
export!(thiscall, rw_00c2c400(this: *const u8) -> f64 {
    unsafe {
        let inner = *(this.add(4) as *const u32);
        if inner != 0 && ((*(inner.wrapping_add(0x28) as *const u32)) & 0x3C0) == 0x80 {
            callee_thiscall!(1, f64, inner.wrapping_add(0x210))
        } else {
            f32::from_bits(*global::<u32>(0xFE8DF8)) as f64
        }
    }
});
