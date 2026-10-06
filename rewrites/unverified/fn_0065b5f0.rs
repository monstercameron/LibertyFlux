// original: 0x0065B5F0 rage::IntervalShadows::vf3

/// Look up one key in the holder's row table; return whether it is present.
///
/// Resolves the 32-bit key through the lookup callee (cdecl: name string,
/// flags 0), then scans the `count` rows (`count` is the unsigned 16-bit word
/// at `+0x0c`, compared as signed but never negative) of 0x30 bytes at the
/// table (`+0x08`), testing the word at `+4` before the word at `+0` of each
/// row. The two compared words and the key are compared for equality only,
/// so their signedness is unobservable. Returns 1 on the first match, else 0
/// (stdcall, one stack argument; the object register is unread).
lf_checker_rt::export!(stdcall, rw_0065b5f0(arg: u32) -> u32 {
    unsafe {
        const HOLDER: u32 = 0x18;
        const ROWS: u32 = 0x08;
        const COUNT: u32 = 0x0c;
        const ROW_STRIDE: u32 = 0x30;
        const KEY_NAME: u32 = 0xFE2E70;
        const CALLEE_LOOKUP: u32 = 1;
        let holder = ((arg + HOLDER) as *const u32).read_unaligned();
        let key: u32 = lf_checker_rt::callee_cdecl!(
            CALLEE_LOOKUP, u32, lf_checker_rt::relocated(KEY_NAME), 0);
        let count = ((holder + COUNT) as *const u16).read_unaligned() as i32;
        let mut row = ((holder + ROWS) as *const u32).read_unaligned().wrapping_add(0x0c);
        let mut idx: i32 = 0;
        while idx < count {
            if ((row + 4) as *const u32).read_unaligned() == key
                || (row as *const u32).read_unaligned() == key
            {
                return 1;
            }
            idx += 1;
            row = row.wrapping_add(ROW_STRIDE);
        }
        0
    }
});
