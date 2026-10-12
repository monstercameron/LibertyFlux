// original: 0x00628700 record_array_copy_backwards (proposed)
/// Copies 64-byte records from one array end to another, top record first.
///
/// `src_start` and `src_end` bound the source range; the record count is the
/// signed difference divided by 64. When it is zero or negative nothing is
/// copied and `dst_end` is returned unchanged. Otherwise that many records
/// ending at `src_end` are copied to end at `dst_end`, starting with the
/// topmost record. Each record copies every byte except three padding words
/// (offsets 26, 34 and 42), which keep the destination's old values.
/// Returns the destination start (`dst_end` minus 64 per record).
///
/// Original: fastcall (`ecx` = src_start, `edx` = src_end, one stack word =
/// dst_end); the count uses an arithmetic shift, so a negative span copies
/// nothing.
lf_checker_rt::export!(fastcall, rw_00628700(src_start: u32, src_end: u32, dst_end: u32) -> u32 {
    const RECORD: u32 = 64;
    let count = src_end.wrapping_sub(src_start) as i32 >> 6;
    if count <= 0 {
        return dst_end;
    }
    unsafe {
        let mut s = src_end as *const u8;
        let mut d = dst_end as *mut u8;
        let mut left = count as u32;
        while left > 0 {
            s = s.sub(RECORD as usize);
            d = d.sub(RECORD as usize);
            core::ptr::copy_nonoverlapping(s, d, 24);
            (d.add(28) as *mut u32).write(((s.add(28)) as *const u32).read_unaligned());
            (d.add(32) as *mut u16).write(((s.add(32)) as *const u16).read_unaligned());
            (d.add(36) as *mut u32).write(((s.add(36)) as *const u32).read_unaligned());
            (d.add(40) as *mut u16).write(((s.add(40)) as *const u16).read_unaligned());
            core::ptr::copy_nonoverlapping(s.add(44), d.add(44), 20);
            left -= 1;
        }
        dst_end.wrapping_sub((count as u32).wrapping_mul(RECORD))
    }
});
