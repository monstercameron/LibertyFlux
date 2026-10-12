// original: 0x008A53C0 audifsound_evaluate_condition

/// Evaluate an if-sound branch condition with a tolerance band.
///
/// Compares the_live float `x2` (first word past `PTR_A`) against the
/// reference `x1` (first word past `PTR_B`, or the inline word `FALLBACK`
/// when that pointer is null), within `EPS` (0.0001, loaded from the image).
/// The unsigned mode word `MODE` selects the test: 0 means x1 - EPS > x2,
/// 1 means x1 + EPS > x2, 2 means x2 > x1 + EPS, 3 means x2 > x1 - EPS,
/// 4 means x2 is inside neither outer band, 5 means x2 is inside the inner
/// band; unordered (NaN) inputs keep the default verdict. Writes 1 then the
/// verdict (0 or 1) to `STATE`, or 2 when the live pointer is null or the
/// mode exceeds 5, and returns the mode (0 on the null path). The float
/// operation order is the original's. Thiscall, object in ECX.
lf_checker_rt::export!(thiscall, rw_008A53C0(this: u32) -> u32 {
    unsafe {
        const PTR_A: u32 = 0xB0;
        const PTR_B: u32 = 0xB8;
        const FALLBACK: u32 = 0xB4;
        const MODE: u32 = 0xBC;
        const STATE: u32 = 0xC0;
        const EPSILON: u32 = 0x00FE_868C;
        const MAX_MODE: u32 = 5;
        let ptr_a = (this.wrapping_add(PTR_A) as *const u32).read_unaligned();
        if ptr_a == 0 {
            (this.wrapping_add(STATE) as *mut u32).write_unaligned(2);
            return 0;
        }
        let x2 = f32::from_bits((ptr_a as *const u32).read_unaligned());
        let ptr_b = (this.wrapping_add(PTR_B) as *const u32).read_unaligned();
        let x1 = if ptr_b == 0 {
            f32::from_bits((this.wrapping_add(FALLBACK) as *const u32).read_unaligned())
        } else {
            f32::from_bits((ptr_b as *const u32).read_unaligned())
        };
        let mode = (this.wrapping_add(MODE) as *const u32).read_unaligned();
        (this.wrapping_add(STATE) as *mut u32).write_unaligned(1);
        if mode > MAX_MODE {
            (this.wrapping_add(STATE) as *mut u32).write_unaligned(2);
            return mode;
        }
        let eps = f32::from_bits(
            (lf_checker_rt::relocated(EPSILON) as *const u32).read_unaligned(),
        );
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let verdict = match mode {
            0 => u32::from(!(sub(x1, eps) > x2)),
            1 => u32::from(!(add(x1, eps) > x2)),
            2 => u32::from(!(x2 > add(x1, eps))),
            3 => u32::from(!(x2 > sub(x1, eps))),
            4 => {
                if x2 > sub(x1, eps) {
                    u32::from(!(add(x1, eps) > x2))
                } else {
                    1
                }
            }
            _ => {
                if x2 > sub(x1, eps) {
                    u32::from(add(x1, eps) > x2)
                } else {
                    0
                }
            }
        };
        if verdict == 0 {
            (this.wrapping_add(STATE) as *mut u32).write_unaligned(0);
        }
        mode
    }
});
