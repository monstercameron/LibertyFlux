// original: 0x009ABA00 audio_float_lookup (proposed)

/// Audio float lookup with override: resolves a table float through a
/// callee, unless a global override flag redirects to a fixed value.
///
/// Loads entry `i` of the global float table `SERIES`, calls the resolver
/// callee with (`this + 0x15b4`, the entry bits) which returns a float in
/// `st0`. When the global byte `OVERRIDE_ON` is nonzero the callee's result
/// is discarded and the global float `OVERRIDE` is returned instead.
/// Returns the float in `st0`.
/// Original: 0x009ABA00 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_009ABA00(this: u32, i: u32) -> f32 {
    unsafe {
        const SERIES: u32 = 0x01168764;
        const GATE: u32 = 0x01289174;
        const OVERRIDE: u32 = 0x01039094;
        const CALLEE_THIS_OFF: u32 = 0x15b4;
        const RESOLVER: u32 = 1;
        let entry = (lf_checker_rt::relocated(SERIES).wrapping_add(i.wrapping_mul(4))
            as *const u32)
            .read_unaligned();
        let r: f32 = lf_checker_rt::callee_thiscall!(
            RESOLVER,
            f32,
            this.wrapping_add(CALLEE_THIS_OFF),
            entry
        );
        if (lf_checker_rt::global::<u8>(GATE)).read() != 0 {
            f32::from_bits(
                (lf_checker_rt::global::<u32>(OVERRIDE) as *const u32).read_unaligned(),
            )
        } else {
            r
        }
    }
});
