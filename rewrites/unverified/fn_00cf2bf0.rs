// original: 0x00CF2BF0 phMaterialMgrGta::vf13
/// Return the address of the requested material record. The receiver stores the table base at byte offset 0x14 and a 16-bit record count at 0x18. The index is unsigned: indices greater than or equal to the count return the unadjusted table base; smaller indices return `base + index * 60` with 32-bit wraparound.
lf_checker_rt::export!(thiscall, rw_00cf2bf0(this: u32, index: u32) -> u32 {
    const TABLE_BASE_FIELD: u32 = 0x14;
    const RECORD_COUNT_FIELD: u32 = 0x18;
    const RECORD_STRIDE: u32 = 60;
    let table_base = unsafe { (this.wrapping_add(TABLE_BASE_FIELD) as *const u32).read_unaligned() };
    let record_count = unsafe { (this.wrapping_add(RECORD_COUNT_FIELD) as *const u16).read_unaligned() as u32 };
    if index >= record_count {
        table_base
    } else {
        table_base.wrapping_add(index.wrapping_mul(RECORD_STRIDE))
    }
});
