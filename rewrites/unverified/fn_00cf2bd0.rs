// original: 0x00CF2BD0 phMaterialMgrGta::vf8
/// Return the signed element index represented by `item` in the material table. The receiver supplies the table base at byte offset 0x14; the stack argument is subtracted from that base with 32-bit wraparound, then divided as a signed value by the 45-byte record stride, truncating toward zero.
lf_checker_rt::export!(thiscall, rw_00cf2bd0(this: u32, item: u32) -> u32 {
    const TABLE_BASE_FIELD: u32 = 0x14;
    const RECORD_STRIDE: i32 = 45;
    let table_base = unsafe { (this.wrapping_add(TABLE_BASE_FIELD) as *const u32).read_unaligned() };
    let byte_delta = item.wrapping_sub(table_base) as i32;
    (byte_delta / RECORD_STRIDE) as u32
});
