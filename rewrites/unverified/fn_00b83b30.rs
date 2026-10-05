// original: 0x00B83B30 subset12_check
/// Test whether every low-12-bit flag in `need` is also set in `have`.
///
/// Scans bits 0..12: any bit set in `need` but clear in `have` fails.
/// Higher bits are ignored. Returns 1 on success, 0 otherwise.
///
/// Original: 0x00B83B30 (stdcall, two stack arguments).
lf_checker_rt::export!(stdcall, rw_00B83B30(need: u32, have: u32) -> u32 {
    unsafe {
        let mut bit = 0u32;
        while bit < 12 {
            let mask = 1u32 << bit;
            if need & mask != 0 && have & mask == 0 {
                return 0;
            }
            bit += 1;
        }
        1
    }
});
