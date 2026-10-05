// original: 0x009aa4a0 table_id_search
/// Find the first Stride25 row whose id word equals `wanted`.
///
/// Scans rows 0..count (the 16-bit count at table `+0x0a`, the table
/// pointer at `this+0x9d0`), comparing the word at row start `+0x0c`
/// with `wanted`. Returns the row number, or -1 when the table is
/// missing, the count is not positive, or no row matches. Thiscall,
/// one stack word, dword result.
export!(thiscall, rw_009AA4A0(this: u32, wanted: u32) -> u32 {
    unsafe {
        const TABLE_PTR: u32 = 0x9d0;
        const ROW_COUNT: u32 = 0x0a;
        const ROW_STRIDE: u32 = 25;
        const ID_OFF: u32 = 0x0c;
        let table = ((this + TABLE_PTR) as *const u32).read_unaligned();
        if table == 0 {
            return 0xFFFFFFFF;
        }
        let count = ((table + ROW_COUNT) as *const u16).read_unaligned() as u32;
        let mut c = 0u32;
        while c < count {
            let id = (table + c * ROW_STRIDE + ID_OFF) as *const u32;
            if id.read_unaligned() == wanted {
                return c;
            }
            c += 1;
        }
        0xFFFFFFFF
    }
});
