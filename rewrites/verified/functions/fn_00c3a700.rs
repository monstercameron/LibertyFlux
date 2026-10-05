// original: 0x00c3a700 nearest_entry_flag (proposed)
/// Flag of the table entry nearest to a value, after wrapping.
///
/// `idx` selects a row and `fbits` is a float bit pattern. `N = counts[idx]`
/// (a signed word table); when `N <= 0` the result is `idx` with its low
/// byte cleared. Otherwise, with `step = spacings[idx]`, each entry
/// `e = xtable[idx][i]` forms `x = e - f`, wrapped up by `step` while above
/// `step/2` and down by `step` while below `-step/2`; the entry with the
/// strictly smallest `|x|` wins (ties keep the earlier one, NaN never
/// wins) and the result is `idx` with its low byte replaced by
/// `bytetable[idx][winner]`. Only the low byte of EAX is written, so the
/// high 24 bits of `idx` pass through.
///
/// Original: 0x00c3a700 (stdcall, two stack words, no calls).
lf_checker_rt::export!(stdcall, rw_00c3a700(idx: u32, fbits: u32) -> u32 {
    unsafe {
        const COUNTS: u32 = 0x16D0870;
        const SPACES: u32 = 0x16D1168;
        const XTAB: u32 = 0x16D08A0;
        const BYTETAB: u32 = 0x16D0C60;
        const HALF: u32 = 0x3f00_0000; // 0.5 (const, embedded)
        const NHALF: u32 = 0xbf00_0000; // -0.5 (const, embedded)
        const BEST0: u32 = 0x4b18_9680; // 1e7 (const, embedded)
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let n = lf_checker_rt::global::<u32>(COUNTS).add(idx as usize).read_unaligned() as i32;
        if n <= 0 {
            return idx & !0xFF;
        }
        let step =
            f32::from_bits(lf_checker_rt::global::<u32>(SPACES).add(idx as usize).read_unaligned());
        let f = f32::from_bits(fbits);
        let hi = fmul(step, f32::from_bits(HALF));
        let lo = fmul(step, f32::from_bits(NHALF));
        let mut best = f32::from_bits(BEST0);
        let mut flag: u32 = 0;
        for ecx in 0..(n as u32) {
            let e = f32::from_bits(
                lf_checker_rt::global::<u32>(XTAB)
                    .add(20 * idx as usize + ecx as usize)
                    .read_unaligned(),
            );
            let mut x = fsub(e, f);
            while x > hi {
                x = fsub(x, step);
            }
            while lo > x {
                x = fadd(x, step);
            }
            let ax = f32::from_bits(x.to_bits() & 0x7FFF_FFFF);
            if best > ax {
                best = ax;
                flag = lf_checker_rt::global::<u8>(BYTETAB)
                    .add(20 * idx as usize + ecx as usize)
                    .read_unaligned() as u32;
            }
        }
        (idx & !0xFF) | flag
    }
});
