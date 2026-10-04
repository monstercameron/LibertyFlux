// original: 0x00c69100 table_rows_search
// Search 42 table rows for a u16 id: each row with a positive count is
// scanned. Returns 1 on the first match, else 0.
export!(stdcall, rw_00c69100(id: u32) -> u32 {
    unsafe {
        let mut row = relocated(0x169c488);
        let mut cntp = relocated(0x169e248);
        let endp = relocated(0x169e2f0);
        loop {
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
            cntp = cntp.wrapping_add(4);
            row = row.wrapping_add(0x50);
            if !(cntp < endp) {
                break;
            }
        }
        0
    }
});
