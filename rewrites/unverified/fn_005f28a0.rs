// original: 0x005f28a0 CRenderPhaseHtml::vf15

/// Forwards this call to the embedded render phase at offset `0x940`. The
/// inherited method returns its result unchanged through EAX.
lf_checker_rt::export!(thiscall, rw_005f28a0(this: u32) -> u32 {
    const EMBEDDED_PHASE: u32 = 0x940;
    lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(EMBEDDED_PHASE))
});
