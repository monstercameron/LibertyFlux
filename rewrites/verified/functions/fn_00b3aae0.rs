// original: 0x00B3AAE0 store_bounds_a

/// Store two 4-float vectors from `a`/`b` into engine-global bound slots and
/// raise the "bounds A present" flag byte.
///
/// Copies `[a, a+16)` to `BOUND0` and `[b, b+16)` to `BOUND1`, then writes 1
/// to `PRESENT`. The moves are bitwise. Cdecl, two stack words, no meaningful
/// return value.
///
/// Original: 0x00B3AAE0.

lf_checker_rt::export!(cdecl, rw_00B3AAE0(a: u32, b: u32) -> u32 {
    unsafe {
        const BOUND0: u32 = 0x01662510;
        const BOUND1: u32 = 0x01662660;
        const PRESENT: u32 = 0x01662491;
        for i in 0..4u32 {
            let v = (a.wrapping_add(i * 4) as *const u32).read_unaligned();
            (lf_checker_rt::global::<u32>(BOUND0).wrapping_add(i as usize) as *mut u32).write_unaligned(v);
        }
        for i in 0..4u32 {
            let v = (b.wrapping_add(i * 4) as *const u32).read_unaligned();
            (lf_checker_rt::global::<u32>(BOUND1).wrapping_add(i as usize) as *mut u32).write_unaligned(v);
        }
        (lf_checker_rt::global::<u8>(PRESENT) as *mut u8).write(1);
        0
    }
});
