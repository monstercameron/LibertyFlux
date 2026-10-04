// original: 0x008f01c0 validate_window_b
/// Validate window B: same window check over its own object offset.
export!(thiscall, rw_008f01c0(this: u32) -> u32 {
    callee_thiscall!(1, u32, this.wrapping_add(0x2F08), 0x3E8, 0x80, 0xFF)
});
