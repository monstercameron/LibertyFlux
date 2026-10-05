// original: 0x008C6EE0 stream_catalog_build
/// Build a wide-character catalog string from the named-entry table.
///
/// Seeds the cursor with the low 16 bits of the measure callee's answer
/// for `dst`, then walks the entry table (count at +0x140 of the table
/// global, names at 0x40-byte strides): an entry whose name, plus one for
/// the separator, still fits in `size` is emitted through two fill-callee
/// calls (separator run, then the name) and the cursor advances past it.
/// Entries that would overflow are skipped. Returns nothing.
/// Original: cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_008c6ee0(dst: u32, size: u32) -> u32 {
    unsafe {
        const MEASURE_CALLEE: u32 = 1;
        const FILL_SEP_CALLEE: u32 = 2;
        const FILL_NAME_CALLEE: u32 = 3;
        const TABLE_FILE_VA: u32 = 0x1BB5624;
        const COUNT_OFF: u32 = 0x140;
        const NAME_STRIDE: u32 = 0x40;
        let n0: u32 = lf_checker_rt::callee_cdecl!(MEASURE_CALLEE, u32, dst);
        let mut cursor = (n0 & 0xFFFF) as u32;
        let table = *lf_checker_rt::global::<u32>(TABLE_FILE_VA);
        let count = ((table + COUNT_OFF) as *const i32).read_unaligned();
        let mut i: i32 = 0;
        while i < count {
            let name = table.wrapping_add(
                (i as u32).wrapping_mul(NAME_STRIDE));
            let mut len: u32 = 0;
            while ((name + len) as *const u8).read() != 0 {
                len += 1;
            }
            if cursor.wrapping_add(1).wrapping_add(len) < size {
                let rest = size.wrapping_sub(cursor);
                let mut scratch: u32 = 0;
                lf_checker_rt::callee_cdecl!(
                    FILL_SEP_CALLEE, u32,
                    dst.wrapping_add(cursor.wrapping_mul(2)),
                    &mut scratch as *mut u32 as u32, rest);
                cursor = cursor.wrapping_add(1);
                lf_checker_rt::callee_cdecl!(
                    FILL_NAME_CALLEE, u32,
                    dst.wrapping_add(cursor.wrapping_mul(2)), name,
                    size.wrapping_sub(cursor));
                cursor = cursor.wrapping_add(len);
            }
            i += 1;
        }
        0
    }
});
