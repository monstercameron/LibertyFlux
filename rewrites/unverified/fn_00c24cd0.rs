// original: 0x00c24cd0 cam_blend_forward (proposed)
/// Reorder `(weight, src, dst, extra)` into `(src, dst, weight, extra)` for
/// the shared blend worker. Entry ECX is discarded; the float travels by
/// value through a vector register with no arithmetic, so all bit patterns
/// survive unchanged.
///
/// Original: 0x00c24cd0 (stdcall, four stack words; first is a float).
lf_checker_rt::export!(stdcall, rw_00c24cd0(weight: u32, src: u32, dst: u32, extra: u32) -> u32 {
    unsafe { lf_checker_rt::callee_cdecl!(1, u32, src, dst, weight, extra) }
});
