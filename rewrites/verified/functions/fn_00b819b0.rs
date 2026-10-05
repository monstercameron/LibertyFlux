// original: 0x00B819B0 script_globals_zero_a
/// Clear three script-runner status words.
///
/// Writes zero to the globals `G0`, `G1` and `G2`. No arguments, no return
/// value, no calls.
///
/// Original: 0x00B819B0 (cdecl).
lf_checker_rt::export!(cdecl, rw_00B819B0() -> u32 {
    unsafe {
        const G0: u32 = 0x0167E2BC;
        const G1: u32 = 0x0167E2C4;
        const G2: u32 = 0x0167E2CC;
        (lf_checker_rt::global::<u32>(G0)).write_unaligned(0);
        (lf_checker_rt::global::<u32>(G1)).write_unaligned(0);
        (lf_checker_rt::global::<u32>(G2)).write_unaligned(0);
        0
    }
});
