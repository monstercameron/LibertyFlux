// original: 0x009e4780 angle_within_limit (proposed)

/// Decide whether a scaled angle is within a float limit, 1 or 0 in `al`.
///
/// Builds four floats (`[q]`, `[q + 4]`, `[leaf + 0x30]`, `[leaf + 0x34]`
/// with `leaf = [this + 0x20]`) in its frame and calls the cdecl dot
/// callee, which reads them as its four stack words and returns a float
/// in ST0. The result is scaled by a constant, wrapped once into
/// `[-WRAP, +WRAP]` (subtract once when above, add once when below zero),
/// and the reference at `[this + 0xaa0]` is wrapped the same way. Returns
/// 1 when the limit exceeds the absolute difference, or when the
/// difference exceeds `WRAP - limit`; else 0. All float comparisons use
/// ordered `comiss` semantics (NaN takes the false side), and only `al`
/// is defined on return. `thiscall`, two stack words (pointer, float).
lf_checker_rt::export!(thiscall, rw_009e4780(this: u32, q: u32, f: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x20;
        const REF_OFF: u32 = 0xaa0;
        const K_MUL: u32 = 0x00fe8728;
        const K_WRAP: u32 = 0x00fe8aec;
        const DOT: u32 = 1;
        let leaf = ((this + LINK) as *const u32).read_unaligned();
        let v0 = ((q) as *const u32).read_unaligned();
        let v1 = ((q.wrapping_add(4)) as *const u32).read_unaligned();
        let v2 = ((leaf.wrapping_add(0x30)) as *const u32).read_unaligned();
        let v3 = ((leaf.wrapping_add(0x34)) as *const u32).read_unaligned();
        let r = lf_checker_rt::callee_cdecl!(DOT, f32, v0, v1, v2, v3);
        let k1 = f32::from_bits(lf_checker_rt::global::<u32>(K_MUL).read());
        let k2 = f32::from_bits(lf_checker_rt::global::<u32>(K_WRAP).read());
        let mut x = core::hint::black_box(r) * core::hint::black_box(k1);
        if x > k2 {
            x = core::hint::black_box(x) - core::hint::black_box(k2);
        }
        if 0.0 > x {
            x = core::hint::black_box(x) + core::hint::black_box(k2);
        }
        let mut y = f32::from_bits(((this + REF_OFF) as *const u32).read_unaligned());
        if y > k2 {
            y = core::hint::black_box(y) - core::hint::black_box(k2);
        }
        if 0.0 > y {
            y = core::hint::black_box(y) + core::hint::black_box(k2);
        }
        let d = core::hint::black_box(x) - core::hint::black_box(y);
        let a = d.abs();
        let lim = f32::from_bits(f);
        if lim > a {
            return 1;
        }
        let k2m = core::hint::black_box(k2) - core::hint::black_box(lim);
        if a > k2m { 1 } else { 0 }
    }
});
