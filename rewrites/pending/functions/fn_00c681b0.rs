// original: 0x00c681b0 table_search_seeded
// Search 42 seeded table rows for a u16 id. A seed pointer must be set; it
// selects the flag table base together with two parameter words. Each row
// with a nonzero flag and a positive count is scanned for the id. Returns
// 1 on the first match, else 0.
export!(stdcall, rw_00c681b0(id: u32) -> u32 {
    unsafe {
        const SEED: u32 = 0x169e3dc;
        const PA: u32 = 0x169c47c;
        const PB: u32 = 0x169c478;
        const COUNTS: u32 = 0x169e320;
        const COUNTS_END: u32 = 0x169e3c8;
        const ROWS: u32 = 0x169d568;
        const ROW_STRIDE: u32 = 0x46;
        const FLAGS: u32 = 0x168ace8;
        let seed = *global::<u32>(SEED);
        if seed == 0 {
            return 0;
        }
        let a = *global::<u32>(PA);
        let b = *global::<u32>(PB);
        let t = (*((seed as *const u8).add(0x20)) & 0x7f) as u32;
        let off = a.wrapping_add(b.wrapping_mul(2)).wrapping_mul(0x47).wrapping_add(t).wrapping_mul(0x2a);
        let mut flagp = relocated(FLAGS).wrapping_add(off);
        let mut cntp = relocated(COUNTS);
        let mut row = relocated(ROWS);
        let endp = relocated(COUNTS_END);
        while cntp < endp {
            if *((flagp) as *const u8) != 0 {
                let n = *(cntp as *const i32);
                if n > 0 {
                    let mut w = row as *const u16;
                    let mut k = 0i32;
                    while k < n {
                        if *w as u32 == id {
                            return 1;
                        }
                        w = w.add(1);
                        k += 1;
                    }
                }
            }
            cntp = cntp.wrapping_add(4);
            flagp = flagp.wrapping_add(1);
            row = row.wrapping_add(ROW_STRIDE);
        }
        0
    }
});
