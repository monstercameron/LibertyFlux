// original: 0x00A4AAE0 vehicle_abs_ge_point8 (proposed)

/// True when the float behind `[this + LINK] + VALUE` lies outside (-0.8, 0.8).
///
/// Loads `obj = [this + LINK]` (0x20), `v = f32[obj + VALUE]` (8), then
/// `v >= HI || LO >= v` with `HI = 0.8`, `LO = -0.8` (constants from `.rdata`,
/// inlined here by value). A NaN fails both IEEE comparisons and returns 0,
/// matching the original's `comiss` unordered path. Pure view; the only
/// result is the low byte.
///
/// Original: 0x00A4AAE0 (thiscall, no stack words), leaf, float compares only.
lf_checker_rt::export!(thiscall, rw_00A4AAE0(this: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x20;
        const VALUE: u32 = 0x08;
        const HI: f32 = f32::from_bits(0x3F4C_CCCD); // 0.8
        const LO: f32 = f32::from_bits(0xBF4C_CCCD); // -0.8
        let obj = ((this + LINK) as *const u32).read_unaligned();
        let v = f32::from_bits(((obj + VALUE) as *const u32).read_unaligned());
        u32::from(v >= HI || LO >= v)
    }
});
