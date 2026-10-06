// original: 0x00a99d20 filemem_limits_exceeded

/// Report whether any of three usage counters exceeds its float limit.
///
/// `this` points to a large object holding three unsigned 32-bit counters
/// (at `+0x8ec40`, `+0x8d830` and `+0x84820`, checked in that order). Each
/// counter is converted to float with a single correct rounding and
/// compared against its limit word; returns 1 on the first counter whose
/// limit is strictly greater than the converted value (an unordered
/// comparison, as with a NaN limit, does not count), else 0.
///
/// Original: 0x00A99D20 (thiscall, no stack arguments, leaf).
lf_checker_rt::export!(thiscall, rw_00a99d20(this: u32) -> u32 {
    unsafe {
        /// (counter offset, limit global file VA) in check order.
        const CHECKS: [(u32, u32); 3] = [
            (0x8ec40, 0x00ea3abc),
            (0x8d830, 0x00ea3ac4),
            (0x84820, 0x00ea3ac8),
        ];
        for (field, limit_va) in CHECKS {
            let raw = (this.wrapping_add(field) as *const u32).read_unaligned();
            let v = raw as f32;
            let lim = (lf_checker_rt::relocated(limit_va) as *const f32).read_unaligned();
            if lim > v {
                return 1;
            }
        }
        0
    }
});
