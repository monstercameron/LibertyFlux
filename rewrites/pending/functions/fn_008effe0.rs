// original: 0x008effe0 stride_table_entry
/// Address of row `obj.row_index` in a global stride table: the row index
/// (a dword at a fixed object offset) scaled by the 188-byte row stride and
/// added to the relocated table base.
export!(thiscall, rw_008effe0(this: u32) -> u32 {
    const ROW_OFF: u32 = 0x32A4;
    const STRIDE: u32 = 0xBC;
    let row = unsafe { *((this.wrapping_add(ROW_OFF)) as *const u32) };
    row.wrapping_mul(STRIDE).wrapping_add(relocated(0x118D470))
});
