// original: 0x00AC6740 stream_clear_flag_bits (proposed)

/// Clear flag bits: `flags &= !(arg & 0x0f)` on the streaming flag byte.
///
/// The original takes one stack word, keeps its low four bits, inverts them
/// and ANDs the result into the flag byte (cdecl). No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC6740(arg: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x0154E042;
        let g = lf_checker_rt::global::<u8>(FLAGS);
        let mask = !((arg as u8) & 0x0f);
        *g = *g & mask;
        0
    }
});
