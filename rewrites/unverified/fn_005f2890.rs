// original: 0x005f2890 CRenderPhaseHtml::vf4

/// Forwards to the structure-copy helper with the destination adjusted by
/// `0xb0` and the source pointer advanced by `0x10`. The target returns the
/// destination pointer through EAX; the original also updates its incoming
/// source-pointer stack slot before tail dispatch.
lf_checker_rt::export!(thiscall, rw_005f2890(this: u32, source: u32) -> u32 {
    const DESTINATION_ADJUST: u32 = 0xb0;
    const SOURCE_ADJUST: u32 = 0x10;
    lf_checker_rt::callee_thiscall!(
        1, u32, this.wrapping_add(DESTINATION_ADJUST), source.wrapping_add(SOURCE_ADJUST)
    )
});
