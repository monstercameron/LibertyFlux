// original: 0x00b05c30 set_flag_and_rescan

/// Stores a narrowed key into the flag table, then rescans for the cursor.
///
/// The high part of `key` selects a table cell whose low 7 bits are replaced
/// by the key's low 7 bits (forced to 1 when 0 or 1); the table's high bits
/// are preserved. The cursor at +0x10 is then set to the first index from 1
/// whose flag byte has the high bit set, or left at 0 when the table head is
/// already flagged (in which case the table address with the new cell byte is
/// returned, matching the original's leftover EAX).
export!(thiscall, rw_00b05c30(obj: *mut u8, key: u32) -> u32 {
    unsafe {
        let table = *(obj.add(4) as *const u32);
        let sel = ((key as i32) >> 8) as u32;
        let cell = table.wrapping_add(sel) as *mut u8;
        *cell &= 0x7f;
        let narrowed = (key & 0x7f) as u8;
        let value = if narrowed <= 1 { 1u8 } else { narrowed };
        let merged = (*cell & 0x80) | value;
        *cell = merged;
        let cursor = obj.add(0x10) as *mut u32;
        *cursor = 0;
        if *(table as *const u8) & 0x80 != 0 {
            return (table & 0xFFFF_FF00) | merged as u32;
        }
        let mut i: u32 = 1;
        loop {
            *cursor = i;
            if *((table.wrapping_add(i)) as *const u8) & 0x80 != 0 {
                return i;
            }
            i = i.wrapping_add(1);
        }
    }
});
