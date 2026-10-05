// original: 0x00E66FC0 task_float_copy_00E66FC0
/// Copy one task float global to another.
///
/// Moves the 32 bits at global `SRC` to global `DST` (a plain bit
/// move, so NaN payloads survive unchanged).
///
/// Original: 0x00E66FC0 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00E66FC0() -> u32 {
    unsafe {
        const SRC: u32 = 0x012BD1D0;
        const DST: u32 = 0x0103B934;
        let v: u32 = (lf_checker_rt::global::<u32>(SRC)).read_unaligned();
        (lf_checker_rt::global::<u32>(DST)).write_unaligned(v);
        0
    }
});
