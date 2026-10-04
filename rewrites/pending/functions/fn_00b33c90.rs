// original: 0x00b33c90 shift_records_down (proposed)

/// Shift an array of 28-byte records towards lower addresses.
///
/// `start` and `end` delimit the source array, `dst_end` the destination end.
/// The record count is the signed count `(end - start) / 28`; when it is not
/// positive nothing is copied and `dst_end` is returned unchanged. Otherwise
/// `count * 28` bytes ending just below `end` are moved to end just below
/// `dst_end`, block by block from the top down. Each 28-byte block moves as
/// three 8-byte halves and one 4-byte tail with the write straight after
/// each read, so an overlapping destination sees partially moved data: this
/// is not `memmove` when the ranges overlap. Returns the destination start
/// (`dst_end - count * 28`).
///
/// Original: 0x00b33c90 (cdecl, three stack words; the count uses the
/// compiler's magic-number signed division by 28).
lf_checker_rt::export!(cdecl, rw_00b33c90(start: u32, end: u32, dst_end: u32) -> u32 {
    unsafe {
        const REC: u32 = 28;
        let count = end.wrapping_sub(start) as i32 / REC as i32;
        if count <= 0 {
            return dst_end;
        }
        let mut s = end;
        let mut d = dst_end;
        for _ in 0..count {
            s = s.wrapping_sub(REC);
            d = d.wrapping_sub(REC);
            let sp = s as *const u8;
            let dp = d as *mut u8;
            (dp as *mut u64).write_unaligned((sp as *const u64).read_unaligned());
            (dp.byte_add(8) as *mut u64)
                .write_unaligned((sp.byte_add(8) as *const u64).read_unaligned());
            (dp.byte_add(16) as *mut u64)
                .write_unaligned((sp.byte_add(16) as *const u64).read_unaligned());
            (dp.byte_add(24) as *mut u32)
                .write_unaligned((sp.byte_add(24) as *const u32).read_unaligned());
        }
        d
    }
});
