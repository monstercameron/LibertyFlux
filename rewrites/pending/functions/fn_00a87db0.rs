// original: 0x00a87db0 stride16_table_store
/// Store a value into a 16-byte-stride table.
///
/// Writes `val` at `this + (idx + 0x89) * 16`. Returns `val`, matching the
/// value the original leaves in EAX.
export!(thiscall, rw_00a87db0(this_obj: u32, idx: u32, val: u32) -> u32 {
    unsafe {
        let row = idx.wrapping_add(0x89).wrapping_mul(2) as usize;
        *((this_obj as usize).wrapping_add(row.wrapping_mul(8)) as *mut u32) = val;
        val
    }
});
