// original: 0x005E8220 CDispatchOrderArrayHandler::vf16

/// Reports whether a dispatch-order row is empty. The handler stores a
/// pointer to the row table at byte offset `0x1864`; each row is `0x50`
/// bytes, and the method's sole stack argument is an unsigned row index.
/// A row is empty when its first word and the 16-bit status at byte offset
/// `0x40` are both zero. The original returns this Boolean in AL.
lf_checker_rt::export!(thiscall, rw_005e8220(this: u32, row_index: u32) -> u8 {
    const ROW_TABLE_POINTER: u32 = 0x1864;
    const ROW_STRIDE: u32 = 0x50;
    const ROW_STATUS: u32 = 0x40;

    unsafe {
        let table = (this.wrapping_add(ROW_TABLE_POINTER) as *const u32).read_unaligned();
        let row = table.wrapping_add(row_index.wrapping_mul(ROW_STRIDE));
        let occupied = (row as *const u32).read_unaligned() != 0;
        let status = (row.wrapping_add(ROW_STATUS) as *const u16).read_unaligned();
        u8::from(!occupied && status == 0)
    }
});
