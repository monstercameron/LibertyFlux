// original: 0x00a90510 stream_table_clear_value

/// Clears every table slot holding a value.
///
/// `this` points to a record with a dword-table pointer at `+0xEC` and a
/// 16-bit slot count at `+0xF0`. Scans slots `0..count` (re-reading the
/// count each step); when `val` is non-zero, every slot equal to `val` is
/// zeroed. Returns the final scan index (the count when stable, 0 when the
/// count starts at 0). No calls, no globals.
/// Original: 0x00A90510 (thiscall, ECX + one stack word), 58 bytes.
lf_checker_rt::export!(thiscall, rw_00a90510(this: u32, val: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0xEC;
        const COUNT_OFF: u32 = 0xF0;
        let count0 = (this.wrapping_add(COUNT_OFF) as *const u16).read_unaligned() as u32;
        if 0u32 >= count0 {
            return 0;
        }
        let mut i = 0u32;
        loop {
            if val != 0 {
                let table = (this.wrapping_add(TABLE_OFF) as *const u32).read_unaligned();
                let slot = table.wrapping_add(i.wrapping_mul(4));
                if (slot as *const u32).read_unaligned() == val {
                    (slot as *mut u32).write_unaligned(0);
                }
            }
            let count = (this.wrapping_add(COUNT_OFF) as *const u16).read_unaligned() as u32;
            i = i.wrapping_add(1);
            if i >= count {
                break;
            }
        }
        i
    }
});
