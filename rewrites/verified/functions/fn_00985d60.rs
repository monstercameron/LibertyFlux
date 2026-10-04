// original: 0x00985d60 audStaticRadioEmitter::vf5
/// Compare a value against the emitter id at `this+0x30`.
///
/// Returns 1 when `v` equals the stored id, else 0.
export!(thiscall, rw_00985d60(this: u32, v: u32) -> u32 {
    unsafe { (*((this.wrapping_add(0x30)) as *const u32) == v) as u32 }
});
