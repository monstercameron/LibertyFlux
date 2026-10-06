// original: 0x0069DE90 clamp_store_pair
/// Clamp two values into [0, limit] (all comparisons signed) and store them.
///
/// `a` is clamped above by the global at LIMIT0 and below at 0, then stored
/// to OUT0; `b` likewise with LIMIT1/OUT1. Both clamps are signed: the upper
/// clamp keeps the value only when limit < value fails as signed (`cmovl`),
/// the lower clamp zeroes it when the value is negative as signed (`cmovs`).
/// Returns 0. Original: cdecl, two stack words, no calls.
lf_checker_rt::export!(cdecl, rw_0069de90(a: u32, b: u32) -> u32 {
    unsafe {
        const LIMIT0: u32 = 0x017A_CCEC;
        const LIMIT1: u32 = 0x017A_CCDC;
        const OUT0: u32 = 0x018B_7A80;
        const OUT1: u32 = 0x018B_7A8C;
        let lim0 = lf_checker_rt::global::<u32>(LIMIT0).read_unaligned();
        let lim1 = lf_checker_rt::global::<u32>(LIMIT1).read_unaligned();
        let mut x = a;
        if (lim0 as i32) < (x as i32) {
            x = lim0;
        }
        if (x as i32) < 0 {
            x = 0;
        }
        lf_checker_rt::global::<u32>(OUT0).write_unaligned(x);
        let mut y = b;
        if (lim1 as i32) < (y as i32) {
            y = lim1;
        }
        if (y as i32) < 0 {
            y = 0;
        }
        lf_checker_rt::global::<u32>(OUT1).write_unaligned(y);
        0
    }
});