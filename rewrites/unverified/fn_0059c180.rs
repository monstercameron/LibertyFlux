// original: 0x0059C180 leaderboard_row_copy
///
/// Copy one leaderboard row from `src` to `dst`, including both counted arrays.
///
/// The header word and the word at +0xC0 are copied directly. Then two array
/// blocks follow: dwords at +4 with a signed count at +0x54, and 8-byte cells
/// at +0x58 with a signed count at +0xB8. Each block is skipped when source
/// and destination arrays are the same pointer (a self copy); otherwise the
/// count is stored to the destination and that many elements are copied, with
/// zero or negative counts copying nothing. Returns the destination pointer.
///
/// thiscall: the destination arrives in ECX, the source on the stack.
lf_checker_rt::export!(thiscall, rw_0059c180(dst: u32, src: u32) -> u32 {
    const WORDS_AT: u32 = 4;
    const WORD_COUNT_AT: u32 = 0x54;
    const CELLS_AT: u32 = 0x58;
    const CELL_COUNT_AT: u32 = 0xB8;
    const TAIL_AT: u32 = 0xC0;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        write_u32(dst, read_u32(src));
        let dst_words = dst.wrapping_add(WORDS_AT);
        let src_words = src.wrapping_add(WORDS_AT);
        if dst_words != src_words {
            let n = read_u32(src_words.wrapping_add(WORD_COUNT_AT)) as i32;
            write_u32(dst_words.wrapping_add(WORD_COUNT_AT), n);
            if n > 0 {
                let mut i = 0u32;
                while i < n as u32 {
                    write_u32(dst_words.wrapping_add(i * 4), read_u32(src_words.wrapping_add(i * 4)));
                    i += 1;
                }
            }
        }
        let dst_cells = dst.wrapping_add(CELLS_AT);
        let src_cells = src.wrapping_add(CELLS_AT);
        if dst_cells != src_cells {
            let m = read_u32(src_cells.wrapping_add(CELL_COUNT_AT)) as i32;
            write_u32(dst_cells.wrapping_add(CELL_COUNT_AT), m as u32);
            if m > 0 {
                let mut i = 0u32;
                while i < m as u32 {
                    let off = i * 8;
                    write_u32(dst_cells.wrapping_add(off), read_u32(src_cells.wrapping_add(off)));
                    write_u32(dst_cells.wrapping_add(off + 4), read_u32(src_cells.wrapping_add(off + 4)));
                    i += 1;
                }
            }
        }
        write_u32(dst.wrapping_add(TAIL_AT), read_u32(src.wrapping_add(TAIL_AT)));
        dst
    }
});
