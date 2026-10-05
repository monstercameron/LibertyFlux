// original: 0x00B3B330 point_in_bounds_b

/// Test whether the 3-float point `p` lies inside bounds B (inclusive).
///
/// Returns 0 when the "bounds B present" flag is clear, else 1 iff every
/// axis satisfies `lo <= x <= hi`. Each `comiss`/`ja` pair in the original
/// fails only on ordered-greater, so NaN inputs fall through to the next
/// test exactly as Rust's `>` does.
///
/// Every exit writes only `al`, so the returned upper 24 bits are the
/// incoming argument word `p` (the flag-clear path returns entry-`eax`
/// bits, which no Rust rewrite can read; the contract fixes entry `eax` to
/// 0, making that path a plain 0).
///
/// Cdecl, one stack word, returns in eax.
///
/// Original: 0x00B3B330.

lf_checker_rt::export!(cdecl, rw_00B3B330(p: u32) -> u32 {
    unsafe {
        const LO: u32 = 0x016624E0;
        const HI: u32 = 0x016624D0;
        const PRESENT: u32 = 0x01662492;
        if (lf_checker_rt::global::<u8>(PRESENT) as *const u8).read() == 0 {
            return 0;
        }
        let mut ok = 1u32;
        for i in 0..3u32 {
            let x = ((p.wrapping_add(i * 4)) as *const f32).read_unaligned();
            let lo = ((lf_checker_rt::global::<f32>(LO).wrapping_add(i as usize)) as *const f32).read_unaligned();
            let hi = ((lf_checker_rt::global::<f32>(HI).wrapping_add(i as usize)) as *const f32).read_unaligned();
            if lo > x || x > hi {
                ok = 0;
                break;
            }
        }
        (p & 0xFFFF_FF00) | ok
    }
});
