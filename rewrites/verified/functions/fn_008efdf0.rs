// original: 0x008efdf0 table_lookup_by_diff
/// Look up a table entry by the gap between an argument and a stored byte.
///
/// When `arg` exceeds the byte at `this + 8`, the index is the stored byte
/// minus `arg` capped at 63, plus 64; otherwise it is the plain difference.
/// Returns the second dword of the selected 8-byte entry from the table at
/// `this + 0x0C`. All arithmetic wraps exactly like the original.
export!(thiscall, rw_008efdf0(this_ptr: u32, arg: u32) -> u32 {
    unsafe {
        let bl = arg as u8;
        let dl = *((this_ptr + 8) as *const u8);
        let idx = if bl > dl {
            let capped = if bl > 0x3F { 0x3Fu32 } else { bl as u32 };
            (dl as u32).wrapping_sub(capped).wrapping_add(0x40) & 0xFF
        } else {
            (dl as u32).wrapping_sub(bl as u32) & 0xFF
        };
        let table = *((this_ptr + 0xC) as *const u32);
        *((table + idx * 8 + 4) as *const u32)
    }
});
