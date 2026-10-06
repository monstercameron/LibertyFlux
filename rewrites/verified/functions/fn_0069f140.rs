// original: 0x0069F140 blend_ratio_from_pair
/// Ratio of two floats with a sign branch, else 0.
///
/// If `a > b` (ordered), returns `(a-b)/(1-b)`; else if `-b > a` (ordered,
/// i.e. `a+b < 0` for finite inputs), returns `(a+b)/(1-b)`; otherwise
/// returns +0.0. Any NaN input takes the ordered-false path at both
/// comparisons and yields 0.0, matching the original's jump-on-unordered.
/// All arithmetic is IEEE single in the original's operand order and pinned.
/// The original stores the result into its own incoming argument slot before
/// loading it onto the x87 stack; the contract switches the stack check off
/// and observes the value through the ST0 return instead. Original: cdecl,
/// two float stack words, no calls.
lf_checker_rt::export!(cdecl, rw_0069f140(a: u32, b: u32) -> f32 {
    unsafe {
        let a = f32::from_bits(a);
        let b = f32::from_bits(b);
        if core::hint::black_box(a) > core::hint::black_box(b) {
            let n = core::hint::black_box(a) - core::hint::black_box(b);
            let d = core::hint::black_box(1.0f32) - core::hint::black_box(b);
            core::hint::black_box(n) / core::hint::black_box(d)
        } else if -core::hint::black_box(b) > core::hint::black_box(a) {
            let n = core::hint::black_box(a) + core::hint::black_box(b);
            let d = core::hint::black_box(1.0f32) - core::hint::black_box(b);
            core::hint::black_box(n) / core::hint::black_box(d)
        } else {
            0.0
        }
    }
});