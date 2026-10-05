// original: 0x00a934d0 stream_scan_rows_dispatch (proposed)

/// Scan the streaming row set, dispatching each live row by a per-row flag.
///
/// Callee 1 fills a 4-byte scratch buffer first (its contents are never
/// read). Then rows 1 through 0x1f3 are visited: callee 2 writes one flag
/// byte; rows are skipped while the global kill flag at `FLAG` is set, while
/// the row's select byte (`selset[i]`, select base at `set+0x04`) has bit 7
/// set, or while the row address (`set+0x00 + stride*i`, stride at
/// `set+0x0c`) is null. A nonzero flag byte dispatches the index to callee 3;
/// a zero flag byte dispatches to callee 4 only when the row's head word is
/// nonzero and its state byte at `+0x54` is nonzero. The flag is a full word:
/// the checker's scripted callee write is word-sized.
///
/// Returns 1 in al (upper bytes are call leftovers). Cdecl, no arguments.
lf_checker_rt::export!(cdecl, rw_00a934d0() -> u32 {
    unsafe {
        const SET_GLOBAL: u32 = 0x012fb258;
        const FLAG_WORD: u32 = 0x0116d27c;
        const FIRST: u32 = 1;
        const LIMIT: u32 = 0x1f4;
        const BASE_OFF: u32 = 0x00;
        const SELECT_OFF: u32 = 0x04;
        const STRIDE_OFF: u32 = 0x0c;
        const STATE_OFF: u32 = 0x54;
        const SKIP_BIT: u8 = 0x80;
        let mut scratch = [0u8; 4];
        lf_checker_rt::callee_cdecl!(1, u32, scratch.as_mut_ptr() as u32, 4);
        let mut i = FIRST;
        while i < LIMIT {
            let mut flag = 0u32;
            lf_checker_rt::callee_cdecl!(2, u32, &mut flag as *mut u32 as u32, 1);
            let killed =
                (lf_checker_rt::global::<u32>(FLAG_WORD).read_unaligned() >> 8) & 0xff;
            if killed == 0 {
                let set = lf_checker_rt::global::<u32>(SET_GLOBAL).read_unaligned();
                let selbase = ((set + SELECT_OFF) as *const u32).read_unaligned();
                if (((selbase + i) as *const u8).read() & SKIP_BIT) == 0 {
                    let stride = ((set + STRIDE_OFF) as *const u32).read_unaligned();
                    let base = ((set + BASE_OFF) as *const u32).read_unaligned();
                    let row = base.wrapping_add(stride.wrapping_mul(i));
                    if row != 0 {
                        if flag != 0 {
                            lf_checker_rt::callee_cdecl!(3, u32, i);
                        } else {
                            let head = (row as *const u32).read_unaligned();
                            if head != 0
                                && ((row + STATE_OFF) as *const u8).read() != 0
                            {
                                lf_checker_rt::callee_cdecl!(4, u32, i);
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        1
    }
});
