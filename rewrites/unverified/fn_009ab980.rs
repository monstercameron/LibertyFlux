// original: 0x009AB980 audio_float_mix (proposed)

/// Audio float mix: blends one scaled series entry with two chained callee
/// results.
///
/// `slot = ([this + 0xc20] + 0x16) mod 30` (division by constant, never
/// faults) picks a float from the series at `this + 0xba8`, scaled by the
/// constant 5.0 (`SCALE`) via `entry * SCALE`. The global float `INPUT` is
/// passed to resolver callee 1 (with `this + 0xca4`), whose `st0` result
/// feeds filter callee 2; the final value is `filter_result + scaled_entry`
/// in that operand order. Float operation order matches the original.
/// Returns the mix in `st0`.
/// Original: 0x009AB980 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_009AB980(this: u32) -> f32 {
    unsafe {
        const COUNT_OFF: u32 = 0xc20;
        const TABLE_BASE: u32 = 0xba8;
        const CYCLE: u32 = 30;
        const PHASE: u32 = 0x16;
        const CALLEE_THIS_OFF: u32 = 0xca4;
        const SCALE: u32 = 0x00fe8ad8;
        const INPUT: u32 = 0x012ddeac;
        const RESOLVER: u32 = 1;
        const FILTER: u32 = 2;

        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let n = ((this.wrapping_add(COUNT_OFF)) as *const u32).read_unaligned();
        let slot = n.wrapping_add(PHASE) % CYCLE;
        let entry = f32::from_bits(
            ((this.wrapping_add(TABLE_BASE).wrapping_add(slot.wrapping_mul(4)))
                as *const u32)
                .read_unaligned(),
        );
        let scale = f32::from_bits(
            (lf_checker_rt::global::<u32>(SCALE) as *const u32).read_unaligned(),
        );
        let scaled = mul(entry, scale);
        let input = f32::from_bits(
            (lf_checker_rt::global::<u32>(INPUT) as *const u32).read_unaligned(),
        );
        let r1: f32 = lf_checker_rt::callee_thiscall!(
            RESOLVER,
            f32,
            this.wrapping_add(CALLEE_THIS_OFF),
            input.to_bits()
        );
        let r2: f32 = lf_checker_rt::callee_cdecl!(FILTER, f32, r1.to_bits());
        add(r2, scaled)
    }
});
