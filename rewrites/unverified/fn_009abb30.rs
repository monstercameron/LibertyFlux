// original: 0x009ABB30 audio_cyclic_float_fetch (proposed)

/// Audio cyclic float fetch: reads one float from a 30-entry ring by a
/// wrapping counter difference.
///
/// `steps = ([this + 0xc20] - sub + 30) mod 30` (all wrapping, `div` by the
/// constant 30 never faults) selects entry `steps` of the float table at
/// `this + 0xba8`. Returns the entry in `st0`.
/// Original: 0x009ABB30 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_009ABB30(this: u32, sub: u32) -> f32 {
    unsafe {
        const COUNT_OFF: u32 = 0xc20;
        const TABLE_BASE: u32 = 0xba8;
        const CYCLE: u32 = 30;
        const PHASE: u32 = 0x1e;
        let base = ((this.wrapping_add(COUNT_OFF)) as *const u32).read_unaligned();
        let steps = base.wrapping_sub(sub).wrapping_add(PHASE) % CYCLE;
        let bits = ((this.wrapping_add(TABLE_BASE).wrapping_add(steps.wrapping_mul(4)))
            as *const u32)
            .read_unaligned();
        f32::from_bits(bits)
    }
});
