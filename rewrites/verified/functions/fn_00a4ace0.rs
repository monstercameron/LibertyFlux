// original: 0x00A4ACE0 vehicle_value_le_neg_point9 (proposed)

/// True when the float behind `[this + LINK] + VALUE` is at most -0.9.
///
/// Loads `obj = [this + LINK]` (0x20), `v = f32[obj + VALUE]` (0x28), then
/// `LO >= v` with `LO = -0.9` (constant from `.rdata`, inlined here by
/// value). A NaN makes the IEEE comparison false and returns 0, matching the
/// original's `comiss` unordered path. Pure view; the only result is the low
/// byte.
///
/// Original: 0x00A4ACE0 (thiscall, no stack words), leaf, one float compare.
lf_checker_rt::export!(thiscall, rw_00A4ACE0(this: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x20;
        const VALUE: u32 = 0x28;
        const LO: f32 = f32::from_bits(0xBF66_6666); // -0.9
        let obj = ((this + LINK) as *const u32).read_unaligned();
        let v = f32::from_bits(((obj + VALUE) as *const u32).read_unaligned());
        u32::from(LO >= v)
    }
});
