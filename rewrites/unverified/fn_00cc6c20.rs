// original: 0x00cc6c20 kind_table_lookup
/// Map a table kind (2, 3, 4) to its info-table address and entry count.
///
/// Writes the table address through `out` and returns the count (12, 10, 3);
/// any other kind yields a null table and a zero count.
export!(stdcall, rw_00cc6c20(kind: u32, out: *mut u32) -> u32 {
    unsafe {
        match kind {
            2 => {
                *out = relocated(0x105152C);
                0xC
            }
            3 => {
                *out = relocated(0x105155C);
                0xA
            }
            4 => {
                *out = relocated(0x1051584);
                3
            }
            _ => {
                *out = 0;
                0
            }
        }
    }
});
