// original: 0x008eef90 find_value_in_id_group
/// Search a packed id's group list for a target value.
///
/// The low word of `id` selects an entry in two global tables: one gives the
/// group header (offset by the high word times 32 bytes), whose low nibble
/// at +0x1E is the list length; the other gives the list base (offset by the
/// header's +0x12 index times 8 bytes). Returns 1 when any of the 8-byte
/// entries' leading dword equals `target`, else 0.
export!(stdcall, rw_008eef90(id: u32, target: u32) -> u8 {
    unsafe {
        let lo = (id & 0xFFFF) as usize;
        let hi = (id >> 16) as usize;
        let headers = global::<u32>(0x1178284);
        let p = (*headers.add(lo)).wrapping_add((hi as u32).wrapping_mul(32));
        let n = (*((p + 0x1E) as *const u8) & 0xF) as usize;
        if n == 0 {
            return 0;
        }
        let lists = global::<u32>(0x1178384);
        let base = *lists.add(lo);
        let start = *((p + 0x12) as *const i16) as isize;
        let mut slot = base.wrapping_add((start as u32).wrapping_mul(8));
        for _ in 0..n {
            if *(slot as *const u32) == target {
                return 1;
            }
            slot = slot.wrapping_add(8);
        }
        0
    }
});
