// original: 0x009f5280 table_bound_check
/// Bound check of an offset against a table base plus a global limit.
/// The low byte carries the boolean; the upper three bytes are whatever the
/// last address computation left in the register, reproduced exactly.
export!(cdecl, rw_009f5280(idx: u32, off: u32) -> u32 {
    // SAFETY: worker maps the original image; table and limit are .data.
    unsafe {
        let base = global::<u32>(0x012B_61D0).add(idx as usize).read();
        if base == 0 {
            return idx & 0xFFFF_FF00;
        }
        let end = off.wrapping_add(base);
        let limit = global::<u32>(0x0117_35B4).read();
        if limit > end {
            end & 0xFFFF_FF00
        } else {
            (end & 0xFFFF_FF00) | 1
        }
    }
});
