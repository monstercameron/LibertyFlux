// original: 0x00a9c910 stream_copy_slot_flags (proposed)

/// Copy one flag byte into each of a counted list of target records.
///
/// `ctx` reaches the work lists through three levels of indirection: the
/// header at `[ctx + 0x40]` points to a row whose first word points to a
/// descriptor. The descriptor holds a 16-bit little-endian count at `+0x1a`
/// (read signed: zero or negative copies nothing and returns the row
/// pointer) and an index array at `+0x10`. Entry `k` takes index `i` from
/// that array, resolves the target through the table at `[[ctx + 8] + 8]`
/// (`target = table[i]`), and stores the low byte of `this + 0x8ec58 + 4k`
/// into the target at `+9`. After the loop the table pointer is returned.
///
/// Original: 0x00a9c910 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a9c910(this: u32, ctx: u32) -> u32 {
    unsafe {
        const HEADER_OFF: u32 = 0x40;
        const COUNT_OFF: u32 = 0x1a;
        const INDEX_ARR_OFF: u32 = 0x10;
        const TABLE_OFF: u32 = 8;
        const FLAGS_OFF: u32 = 0x8ec58;
        const TARGET_FLAG_OFF: u32 = 9;
        let row = ((ctx + HEADER_OFF) as *const u32).read_unaligned();
        let desc = (row as *const u32).read_unaligned();
        let count = ((desc + COUNT_OFF) as *const u16).read_unaligned() as i16 as i32;
        if count <= 0 {
            return desc;
        }
        let mut k = 0u32;
        while (k as i32) < count {
            let indices = ((desc + INDEX_ARR_OFF) as *const u32).read_unaligned();
            let index =
                ((indices + k.wrapping_mul(2)) as *const u16).read_unaligned() as u32;
            let mid = ((ctx + TABLE_OFF) as *const u32).read_unaligned();
            let table = ((mid + TABLE_OFF) as *const u32).read_unaligned();
            let target = ((table + index.wrapping_mul(4)) as *const u32).read_unaligned();
            let flag = ((this + FLAGS_OFF + k.wrapping_mul(4)) as *const u8).read();
            ((target + TARGET_FLAG_OFF) as *mut u8).write(flag);
            k += 1;
        }
        let mid = ((ctx + TABLE_OFF) as *const u32).read_unaligned();
        ((mid + TABLE_OFF) as *const u32).read_unaligned()
    }
});
