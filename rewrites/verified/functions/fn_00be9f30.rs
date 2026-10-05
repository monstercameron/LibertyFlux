// original: 0x00be9f30 table_first_zero_word (proposed)

/// Scan a 256-entry strided table for the first entry holding a zero word.
///
/// Takes the start index. Returns 0 at once for an index at or above 0x100.
/// Otherwise walks entries of 0xE0 bytes from the table at file address
/// `0x12E25F8`: at each entry, returns the address of `[e-8]` when it is
/// zero, else of `[e]` when that is zero, else of `[e-4]` when that is zero,
/// else advances to the next entry, returning 0 after the last one. A result
/// address that computes to null is reported as 0 (unreachable in practice:
/// the table base is far from null, recorded in the proof notes). No calls.
///
/// Original: cdecl, one stack word, plain `ret`, returns `eax`.
lf_checker_rt::export!(cdecl, rw_00be9f30(start: u32) -> u32 {
    unsafe {
        const TABLE_FILE: u32 = 0x12E25F8;
        const STRIDE: u32 = 0xE0;
        const COUNT: u32 = 0x100;

        let mut index = start;
        if index >= COUNT {
            return 0;
        }
        let base = lf_checker_rt::relocated(TABLE_FILE);
        loop {
            let e = base.wrapping_add(index.wrapping_mul(STRIDE));
            if ((e.wrapping_sub(8)) as *const u32).read_unaligned() == 0 {
                return e.wrapping_sub(8);
            }
            if (e as *const u32).read_unaligned() == 0 {
                return e;
            }
            if ((e.wrapping_sub(4)) as *const u32).read_unaligned() == 0 {
                return e.wrapping_sub(4);
            }
            index = index.wrapping_add(1);
            if index >= COUNT {
                return 0;
            }
        }
    }
});
