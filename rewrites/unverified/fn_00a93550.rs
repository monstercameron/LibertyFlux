// original: 0x00a93550 stream_scan_rows_report (proposed)

/// Scan the streaming row set, reporting each row's liveness to callee 2.
///
/// Callee 1 fills a 4-byte scratch buffer first (its contents are never
/// read; the constant the original also stores to its frame is dead too).
/// Then rows 1 through 0x1f3 are visited: a row is live when its select byte
/// (`selset[i]`, select base at `set+0x04`) has bit 7 clear, its address
/// (`set+0x00 + stride*i`, stride at `set+0x0c`) is non-null, its head word
/// is nonzero and its state byte at `+0x54` is nonzero. The verdict byte (1
/// for live, 0 otherwise) is passed to callee 2 together with the length 1.
///
/// Returns 1 in al (upper bytes are call leftovers). Cdecl, no arguments.
lf_checker_rt::export!(cdecl, rw_00a93550() -> u32 {
    unsafe {
        const SET_GLOBAL: u32 = 0x012fb258;
        const FIRST: u32 = 1;
        const LIMIT: u32 = 0x1f4;
        const BASE_OFF: u32 = 0x00;
        const SELECT_OFF: u32 = 0x04;
        const STRIDE_OFF: u32 = 0x0c;
        const STATE_OFF: u32 = 0x54;
        const SKIP_BIT: u8 = 0x80;
        let mut scratch = [0u8; 4];
        lf_checker_rt::callee_cdecl!(1, u32, scratch.as_mut_ptr() as u32, 4);
        let set = lf_checker_rt::global::<u32>(SET_GLOBAL).read_unaligned();
        let mut i = FIRST;
        while i < LIMIT {
            let selbase = ((set + SELECT_OFF) as *const u32).read_unaligned();
            let mut live = 0u8;
            if (((selbase + i) as *const u8).read() & SKIP_BIT) == 0 {
                let stride = ((set + STRIDE_OFF) as *const u32).read_unaligned();
                let base = ((set + BASE_OFF) as *const u32).read_unaligned();
                let row = base.wrapping_add(stride.wrapping_mul(i));
                if row != 0
                    && (row as *const u32).read_unaligned() != 0
                    && ((row + STATE_OFF) as *const u8).read() != 0
                {
                    live = 1;
                }
            }
            lf_checker_rt::callee_cdecl!(2, u32, &live as *const u8 as u32, 1);
            i += 1;
        }
        1
    }
});
