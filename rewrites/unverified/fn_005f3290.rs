// original: 0x005f3290 GetIndexedObjectPayloadA

/// Selects an object through the pointer table at `this`, loads that object's
/// nested payload record at `+0xa0`, then returns the first payload word when
/// the key at `this + 4` equals the nested record's key at `+0x34`; otherwise
/// it returns the alternate payload word. The table index is at `this + 8`.
lf_checker_rt::export!(thiscall, rw_005f3290(this: u32) -> u32 {
    const OBJECT_TABLE: u32 = 0x00;
    const OBJECT_KEY: u32 = 0x04;
    const TABLE_INDEX: u32 = 0x08;
    const NESTED_RECORD: u32 = 0xa0;
    const RECORD_KEY: u32 = 0x34;
    const PRIMARY_VALUE: u32 = 0x68;
    const ALTERNATE_VALUE: u32 = 0x70;

    let table = unsafe { (this.wrapping_add(OBJECT_TABLE) as *const u32).read_unaligned() };
    let index = unsafe { (this.wrapping_add(TABLE_INDEX) as *const u32).read_unaligned() };
    let object_slot = table.wrapping_add(index.wrapping_mul(4));
    let object = unsafe { (object_slot as *const u32).read_unaligned() };
    let record = unsafe { (object.wrapping_add(NESTED_RECORD) as *const u32).read_unaligned() };
    let requested_key = unsafe { (this.wrapping_add(OBJECT_KEY) as *const u32).read_unaligned() };
    let stored_key = unsafe { (record.wrapping_add(RECORD_KEY) as *const u32).read_unaligned() };
    let value_offset = if requested_key == stored_key { PRIMARY_VALUE } else { ALTERNATE_VALUE };
    unsafe { (record.wrapping_add(value_offset) as *const u32).read_unaligned() }
});
