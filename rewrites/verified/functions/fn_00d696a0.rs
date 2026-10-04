// original: 0x00d696a0 prev_entry_or_null
// s16f07: fetch the previous table entry of this record, or null
// (thiscall/0). Same one-based rule as s16f06 but the table links live on
// this record directly instead of behind a child pointer.
export!(thiscall, rw_s16f07(this: *const u8) -> u32 {
    unsafe {
        let pos = *((this.add(0xA0)) as *const u32);
        if (pos.wrapping_sub(1) as i32) < 0 {
            return 0;
        }
        let holder = *((this.add(0x9C)) as *const u32);
        let table = *(holder as *const u32);
        *((table.wrapping_add(pos.wrapping_mul(4).wrapping_sub(4))) as *const u32)
    }
});
