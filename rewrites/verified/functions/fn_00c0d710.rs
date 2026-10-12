// original: 0x00c0d710 UILayoutFrame::vf35

/// Adjusts the UILayoutFrame receiver by 0xA4 bytes and forwards it to the
/// shared mainloop timing-scale implementation. The implementation returns an
/// exact single-precision value in the x87 register stack.
lf_checker_rt::export!(thiscall, rw_c0d710(this: u32) -> f32 {
    const RECEIVER_ADJUSTMENT: u32 = 0xA4;
    const TIMING_SCALE_HELPER: u32 = 1;
    let adjusted_this = this.wrapping_add(RECEIVER_ADJUSTMENT);
    lf_checker_rt::callee_thiscall!(TIMING_SCALE_HELPER, f32, adjusted_this)
});
