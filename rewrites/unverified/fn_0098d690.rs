// original: 0x0098D690 id_table_contains

/// Searches `count` signed-positive halfword entries in the row selected by
/// `index`. Each row is 0x46 bytes. The count is signed: zero or a negative
/// count returns false without reading the row. The low byte reports whether
/// `query` was found; the upper return bytes preserve the original array
/// cursor (or the input index when the count is non-positive).
lf_checker_rt::export!(cdecl, rw_id_table_contains(index: u32, query: u32) -> u32 {
    const ITEM_COUNTS: u32 = 0x169E_320;
    const ITEM_ROWS: u32 = 0x169D_568;
    const ROW_BYTES: usize = 0x46;
    const LOW_BYTE: u32 = 0xFF;

    unsafe {
        let count = lf_checker_rt::global::<i32>(ITEM_COUNTS)
            .add(index as usize)
            .read_unaligned();
        if count <= 0 {
            return index & !LOW_BYTE;
        }

        let row = lf_checker_rt::global::<u16>(ITEM_ROWS)
            .add(index as usize * (ROW_BYTES / 2));
        let mut cursor = row;
        let mut result = cursor as u32;
        let mut position = 0i32;
        while position < count {
            if u32::from(cursor.read_unaligned()) == query {
                result = cursor as u32;
                result = (result & !LOW_BYTE) | 1;
                return result;
            }
            cursor = cursor.add(1);
            result = cursor as u32;
            position += 1;
        }
        result & !LOW_BYTE
    }
});
