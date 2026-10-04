// original: 0x00E66DC0 copy_3_globals

/// Copy three global words to three other globals, one by one.
///
/// Copies the 32-bit values at `SRC0..SRC2` to `DST0..DST2`. The original
/// moves them through an SSE register (`movss`), which is a plain 32-bit copy
/// with no canonicalisation, so the rewrite copies raw `u32` bits.
///
/// Original: 0x00E66DC0 (cdecl, no arguments, no outgoing calls, no return value).
lf_checker_rt::export!(cdecl, rw_00e66dc0() -> u32 {
    unsafe {
        const SRC0: u32 = 0x01B4B2A0;
        const SRC1: u32 = 0x01B4B2A4;
        const SRC2: u32 = 0x01B4B2A8;
        const DST0: u32 = 0x012B6290;
        const DST1: u32 = 0x012B6294;
        const DST2: u32 = 0x012B6298;
        for (s, d) in [(SRC0, DST0), (SRC1, DST1), (SRC2, DST2)] {
            let v = (lf_checker_rt::global::<u32>(s)).read_unaligned();
            (lf_checker_rt::global::<u32>(d)).write_unaligned(v);
        }
    }
    0
});
