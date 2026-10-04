// original: 0x00a712f0 CTaskComplexPlayerInCover::vf18 (symbols)
/// Forwards one argument to the base-class handler, then returns 0.
///
/// `thiscall`: object in ECX (passed through untouched), one stack word,
/// callee pops 4. The callee's answer is discarded; the return is always 0.
lf_checker_rt::export!(thiscall, rw_00a712f0(this: u32, arg: u32) -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, this, arg);
    0
});
