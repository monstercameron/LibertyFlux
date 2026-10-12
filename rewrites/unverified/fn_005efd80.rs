// original: 0x005EFD80 cooldown_tick_reset (proposed)

/// Refresh a cooldown slot while the shared limit stays above its value.
///
/// Compares the shared limit (a float in image data) against the slot's
/// value at `this + 0xe08`. When the limit is strictly greater, increments
/// the counter at `this + 0xe04` and resets the value to 15.0. Otherwise
/// (limit less than, equal, or unordered against the value, which covers
/// not-a-number on either side) leaves the slot untouched. The original is
/// thiscall with no stack arguments and no meaningful return value.
lf_checker_rt::export!(thiscall, rw_005efd80(this: u32) -> u32 {
    unsafe {
        const LIMIT_GLOBAL: u32 = 0x00FE8B1C;
        const COUNT_OFFSET: u32 = 0x0e04;
        const VALUE_OFFSET: u32 = 0x0e08;
        const RESET_BITS: u32 = 0x41700000; // 15.0f
        let limit = f32::from_bits(
            (lf_checker_rt::global::<u32>(LIMIT_GLOBAL)).read_unaligned()
        );
        let current =
            (this.wrapping_add(VALUE_OFFSET) as *const f32).read_unaligned();
        if limit > current {
            let slot = this.wrapping_add(COUNT_OFFSET) as *mut u32;
            slot.write_unaligned(slot.read_unaligned().wrapping_add(1));
            (this.wrapping_add(VALUE_OFFSET) as *mut u32)
                .write_unaligned(RESET_BITS);
        }
        0
    }
});
