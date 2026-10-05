// original: 0x00a3b3a0 vehicle_lane_value (proposed)

/// Fetch slot `+0x15c` of the indexed lane record, or 0.
///
/// The record array lives at `[rec + 0xf80]` with `0x170`-byte rows and
/// `[rec + 0xf84]` rows, where `rec = *obj`. An index at or above the
/// count (signed) answers 0.0, as does a row address that computes to
/// null. (A second, identical bounds check past the null test can never
/// fail and its fault path is dead.) Thiscall/1, returns the float on
/// x87 ST0.
lf_checker_rt::export!(thiscall, rw_00a3b3a0(obj: u32, idx: u32) -> f32 {
    unsafe {
        const ROWS: u32 = 0xF80;
        const COUNT: u32 = 0xF84;
        const STRIDE: i32 = 0x170;
        const SLOT: u32 = 0x15C;
        let rec = core::ptr::read_unaligned(obj as *const u32);
        let count = core::ptr::read_unaligned((rec + COUNT) as *const u32) as i32;
        if (idx as i32) >= count {
            return 0.0;
        }
        let base = core::ptr::read_unaligned((rec + ROWS) as *const u32);
        let row = base.wrapping_add(((idx as i32).wrapping_mul(STRIDE)) as u32);
        if row == 0 {
            return 0.0;
        }
        f32::from_bits(core::ptr::read_unaligned((row + SLOT) as *const u32))
    }
});
