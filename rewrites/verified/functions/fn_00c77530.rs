// original: 0x00C77530 streaming_entry_address
/// Return the indexed streaming-row address.
///
/// `index` selects a flag byte and row using the bases and stride held in the
/// streaming table. If the selected flag has bit `0x80` set, the function
/// returns the fixed offset `0x10`; otherwise it returns the row base plus
/// `index * stride + 0x10`, with 32-bit wrapping arithmetic. The flag test is
/// performed before the row calculation. The original is cdecl with one
/// 32-bit stack argument and returns the address or offset in EAX.
///
#[unsafe(no_mangle)]
pub extern "cdecl" fn rw_00c77530(index: u32) -> u32 {
    const STREAMING_TABLE_FILE_VA: u32 = 0x016E_C774;
    const TABLE_ROW_BASE: usize = 0x00;
    const TABLE_FLAG_BASE: usize = 0x04;
    const TABLE_ROW_STRIDE: usize = 0x0C;
    const INVALID_ROW_FLAG: u8 = 0x80;
    const ROW_FIELD_OFFSET: u32 = 0x10;
    const MUTANT_FIELD_OFFSET: u32 = 0x14;

    unsafe {
        let table_address = lf_checker_rt::global::<u32>(STREAMING_TABLE_FILE_VA)
            .read_unaligned() as usize;
        let row_base = (table_address.wrapping_add(TABLE_ROW_BASE) as *const u32)
            .read_unaligned();
        let flag_base = (table_address.wrapping_add(TABLE_FLAG_BASE) as *const u32)
            .read_unaligned() as usize;
        let row_stride = (table_address.wrapping_add(TABLE_ROW_STRIDE) as *const u32)
            .read_unaligned();
        let flags = (flag_base.wrapping_add(index as usize) as *const u8).read();
        if flags & INVALID_ROW_FLAG != 0 {
            return ROW_FIELD_OFFSET;
        }
        let row_offset = index.wrapping_mul(row_stride);
        row_base
            .wrapping_add(row_offset)
            .wrapping_add(ROW_FIELD_OFFSET)
    }
}
