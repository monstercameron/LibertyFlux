// original: 0x00b9b1d0 NativeImpl_DEFINE_PED_GENERATION_CONSTRAINT_AREA

/// Defines a ped-generation constraint area from four float arguments.
///
/// Copies (`x`, `y`, `z`) into an aligned frame buffer of
/// [x, y, z, scratch] (the fourth word is frame scratch, zero under the
/// contract's `stack_fill`) and passes a pointer to it through the single
/// `DEFINE` call. The square of the fourth float (`r * r`, single
/// precision) is staged in dead scratch below the buffer and never read.
/// Returns whatever the callee returns.
///
/// The buffer pointer is a skipped call argument with snapshot-verified
/// contents (see `narrowed`). The square pins the original's operand
/// order (both operands are the same value).
///
/// Original: 0x00B9B1D0 (cdecl, four stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9b1d0(x: u32, y: u32, z: u32, r: u32) -> u32 {
    const DEFINE: u32 = 1;
    #[inline(always)]
    fn fmul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    let _dead = fmul(f32::from_bits(r), f32::from_bits(r));
    let mut buf = [x, y, z, 0];
    lf_checker_rt::callee_cdecl!(DEFINE, u32, buf.as_mut_ptr() as u32)
});
