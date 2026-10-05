// original: 0x00B3B2B0 point_in_bounds_or_vol

/// Test the point `p` against volume callee 1, then against bounds A.
///
/// Callee 1 tests the point; a non-zero low byte in its answer accepts. Else
/// the "bounds A present" flag must be set and every axis must satisfy
/// `lo <= x <= hi` (bounds A globals). Every exit writes only `al`, so the
/// returned upper 24 bits are always callee 1's answer bits. Cdecl, one
/// stack word, returns in eax.
///
/// Original: 0x00B3B2B0.

lf_checker_rt::export!(cdecl, rw_00B3B2B0(p: u32) -> u32 {
    unsafe {
        const VOL: u32 = 1;
        const LO: u32 = 0x01662510;
        const HI: u32 = 0x01662660;
        const PRESENT: u32 = 0x01662491;
        let v: u32 = lf_checker_rt::callee_cdecl!(VOL, u32, p);
        let bit = if (v & 0xFF) != 0 {
            1
        } else if (lf_checker_rt::global::<u8>(PRESENT) as *const u8).read() == 0 {
            0
        } else {
            let mut ok = 1u32;
            for i in 0..3u32 {
                let x = (p.wrapping_add(i * 4) as *const f32).read_unaligned();
                let lo =
                    (lf_checker_rt::global::<f32>(LO).wrapping_add(i as usize) as *const f32).read_unaligned();
                let hi =
                    (lf_checker_rt::global::<f32>(HI).wrapping_add(i as usize) as *const f32).read_unaligned();
                if lo > x || x > hi {
                    ok = 0;
                    break;
                }
            }
            ok
        };
        (v & 0xFFFF_FF00) | bit
    }
});
