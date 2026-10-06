// original: 0x00697ca0 weight_probe (proposed)

/// Probe each key of the table, writing 1.0 or 0.0 per answer.
///
/// `this` points at a descriptor: the key table at `+0` (entry array at
/// `+0x0c`, 16-bit signed count at `+0x10`), the prober object at `+4`, and a
/// float cursor at `+8`. For each of the `count` entries (nothing happens
/// when the signed count is not positive) the cursor word is set to 1.0, the
/// prober's vtable slot 3 is asked about the entry key (high byte at `+5`
/// passed low-byte-only, low word at `+6`, cursor address), and a zero low
/// answer byte resets the cursor word to 0.0. Both cursors advance per entry.
/// Returns nothing significant (the last answer byte, or entry `eax` when the
/// count is empty).
///
/// Original: thiscall, two stack words it pops but never reads, callee
/// cleans 8.
lf_checker_rt::export!(thiscall, rw_00697ca0(this: u32, _u1: u32, _u2: u32) -> u32 {
    unsafe {
        const TABLE_ARRAY: u32 = 0x0c;
        const TABLE_COUNT: u32 = 0x10;
        const KEY_HI: u32 = 5;
        const KEY_LO: u32 = 6;
        const PROBE_SLOT: u32 = 0x0c;
        const ONE: u32 = 0x3f800000;
        let _ = (_u1, _u2);
        let table = (this as *const u32).read_unaligned();
        let count =
            (table as *const u16).byte_offset(TABLE_COUNT as isize).read_unaligned() as i32;
        if count <= 0 {
            return 0;
        }
        let mut array =
            (table as *const u32).byte_offset(TABLE_ARRAY as isize).read_unaligned();
        let obj = (this as *const u32).byte_offset(4).read_unaligned();
        let vtable = (obj as *const u32).read_unaligned();
        let routine =
            (vtable as *const u32).byte_offset(PROBE_SLOT as isize).read_unaligned();
        let probe: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(routine as usize);
        let mut remaining = count as u32;
        while remaining != 0 {
            let entry = (array as *const u32).read_unaligned();
            let cursor = (this as *const u32).byte_offset(8).read_unaligned();
            (cursor as *mut u32).write_unaligned(ONE);
            let key_lo =
                (entry as *const u16).byte_offset(KEY_LO as isize).read_unaligned() as u32;
            let key_hi = (entry as *const u8).byte_offset(KEY_HI as isize).read() as u32;
            let answer = probe(obj, key_hi, key_lo, cursor);
            if answer & 0xff == 0 {
                (cursor as *mut u32).write_unaligned(0);
            }
            let next_cursor = cursor.wrapping_add(4);
            (this as *mut u32).byte_offset(8).write_unaligned(next_cursor);
            array = array.wrapping_add(4);
            remaining -= 1;
        }
        0
    }
});
