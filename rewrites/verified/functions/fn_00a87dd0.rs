// original: 0x00a87dd0 stride16_table_store_off8
/// Store a value into the second slot of a 16-byte table row.
///
/// Writes `val` at `this + idx * 16 + 0x898`. Returns `val`, matching the
/// value the original leaves in EAX.
export!(thiscall, rw_00a87dd0(this_obj: u32, idx: u32, val: u32) -> u32 {
    unsafe {
        let off = (idx as usize).wrapping_mul(2).wrapping_mul(8).wrapping_add(0x898);
        *((this_obj as usize).wrapping_add(off) as *mut u32) = val;
        val
    }
});
