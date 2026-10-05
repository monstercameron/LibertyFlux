// original: 0x00c093a0 stream_row_append (proposed)

/// Append one row to the streaming table when it has room.
///
/// `this` points to the table (`COUNT` its 16-bit length, `CAP` its capacity,
/// `ROWS` the row array, each row `ROW_STRIDE` bytes). When the count is
/// already at capacity (signed compare) nothing happens and the count is
/// returned. Otherwise the next row is filled from the arguments: the three
/// floats at `p0`, the float `f0`, the three floats at `p1`, the float `f1`,
/// the count is advanced by one and the new count is returned.
///
/// Original: 0x00c093a0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00c093a0(this: u32, p0: u32, p1: u32, f0: u32, f1: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x5228;
        const CAP: u32 = 0x5220;
        const ROWS: u32 = 0x5224;
        const ROW_STRIDE: u32 = 32;
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        let cap = (this.wrapping_add(CAP) as *const u32).read_unaligned();
        if (count as i32) >= (cap as i32) {
            return count;
        }
        let rows = (this.wrapping_add(ROWS) as *const u32).read_unaligned();
        let row = rows.wrapping_add(count.wrapping_mul(ROW_STRIDE));
        let mut o = 0u32;
        while o < 3 {
            let v = (p0.wrapping_add(o.wrapping_mul(4)) as *const u32).read_unaligned();
            (row.wrapping_add(o.wrapping_mul(4)) as *mut u32).write_unaligned(v);
            o += 1;
        }
        (row.wrapping_add(12) as *mut u32).write_unaligned(f0);
        let mut o = 0u32;
        while o < 3 {
            let v = (p1.wrapping_add(o.wrapping_mul(4)) as *const u32).read_unaligned();
            (row.wrapping_add(16).wrapping_add(o.wrapping_mul(4)) as *mut u32).write_unaligned(v);
            o += 1;
        }
        (row.wrapping_add(28) as *mut u32).write_unaligned(f1);
        let next = count.wrapping_add(1);
        (this.wrapping_add(COUNT) as *mut u16).write_unaligned(next as u16);
        next
    }
});
