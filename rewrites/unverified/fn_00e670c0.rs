// original: 0x00E670C0 task_float_copy_00E670C0
/// Copy one task float global to another.
///
/// Moves the 32 bits at global `SRC` to global `DST` (a plain bit
/// move, so NaN payloads survive unchanged).
///
/// Original: 0x00E670C0 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00E670C0() -> u32 {
    unsafe {
        const SRC: u32 = 0x01048AB4;
        const DST: u32 = 0x0103C060;
        let v: u32 = (lf_checker_rt::global::<u32>(SRC)).read_unaligned();
        (lf_checker_rt::global::<u32>(DST)).write_unaligned(v);
        0
    }
});
