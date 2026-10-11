// original: 0x005E8250 CDispatchOrderArrayHandler::vf18

/// Selects a dispatch-order row through the handler's table pointer at byte
/// offset `0x1864`, using rows with a `0x50`-byte stride, then returns the
/// value produced by the row's query helper. The method is thiscall with one
/// unsigned row index; the selected row is the helper's this pointer, and
/// the helper's EAX result is returned unchanged.
lf_checker_rt::export!(thiscall, rw_005e8250(this: u32, row_index: u32) -> u32 {
    const ROW_TABLE_POINTER: u32 = 0x1864;
    const ROW_STRIDE: u32 = 0x50;

    let table = unsafe { (this.wrapping_add(ROW_TABLE_POINTER) as *const u32).read_unaligned() };
    let row = table.wrapping_add(row_index.wrapping_mul(ROW_STRIDE));
    lf_checker_rt::callee_thiscall!(1, u32, row)
});
