// original: 0x00c3d850 train_clear_low_nibble_guarded (proposed)
/// Clear the low nibble of the flag byte at `+0x14e9` unless guards say stop.
///
/// Returns without writing when the global dword G1 is 1, when the global
/// A differs from the constant B, or when the global M is 0x12; otherwise
/// ANDs the byte with 0xf0. The contract cycles the globals through all
/// four paths (write, and each early exit). Returns nothing meaningful.
///
/// Original: 0x00c3d850 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c3d850(this: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x14e9;
        const KEEP: u8 = 0xf0;
        const G1: u32 = 0x11f7060;
        const A: u32 = 0x12088b4;
        const B: u32 = 0xf1c040;
        const M: u32 = 0x1037720;
        const SKIP_M: u32 = 0x12;
        if lf_checker_rt::global::<u32>(G1).read_unaligned() == 1 {
            return 0;
        }
        let a = lf_checker_rt::global::<u32>(A).read_unaligned();
        let b = lf_checker_rt::global::<u32>(B).read_unaligned();
        if a != b {
            return 0;
        }
        if lf_checker_rt::global::<u32>(M).read_unaligned() == SKIP_M {
            return 0;
        }
        let f = ((this + FLAGS) as *const u8).read();
        ((this + FLAGS) as *mut u8).write(f & KEEP);
        0
    }
});
