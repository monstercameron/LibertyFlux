// original: 0x00E67010 task_float_copy_00E67010
/// Copy one task float global to another.
///
/// Moves the 32 bits at global `SRC` to global `DST` (a plain bit
/// move, so NaN payloads survive unchanged).
///
/// Original: 0x00E67010 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00E67010() -> u32 {
    unsafe {
        const SRC: u32 = 0x01048AB4;
        const DST: u32 = 0x012BD1D0;
        let v: u32 = (lf_checker_rt::global::<u32>(SRC)).read_unaligned();
        (lf_checker_rt::global::<u32>(DST)).write_unaligned(v);
        0
    }
});
