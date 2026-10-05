// original: 0x0062C4B0 rage::VerletWaterSurface::VerletWaterSurfaceVarCache::vf0

/// Cache two row indexes: the shared scan twice, storing each index + 1.
///
/// Runs the row scan of the `vf3` family twice over the same holder with two
/// lookup keys (name strings `KEY_A` and `KEY_B`): the 1-based position of
/// the first match, or 0, is stored at `OUT_A` (`+4` of `this`) and `OUT_B`
/// (`+8`). Returns the second stored value on a match and the second scan
/// count otherwise, as the original leaves it (thiscall, one argument).
lf_checker_rt::export!(thiscall, rw_0062c4b0(this: u32, arg: u32) -> u32 {
    unsafe {
        const HOLDER: u32 = 0x18;
        const ROWS: u32 = 0x08;
        const COUNT: u32 = 0x0c;
        const ROW_STRIDE: u32 = 0x30;
        const OUT_A: u32 = 0x04;
        const OUT_B: u32 = 0x08;
        const KEY_A: u32 = 0xFE2398;
        const KEY_B: u32 = 0xFE23A4;
        unsafe fn scan(holder: u32, key: u32) -> (u32, u32) {
            unsafe {
                let count = ((holder + COUNT) as *const u16).read_unaligned() as i32;
                let mut row =
                    ((holder + ROWS) as *const u32).read_unaligned().wrapping_add(0x0c);
                let mut idx: i32 = 0;
                while idx < count {
                    if ((row + 4) as *const u32).read_unaligned() == key
                        || (row as *const u32).read_unaligned() == key
                    {
                        let pos = (idx + 1) as u32;
                        return (pos, pos);
                    }
                    idx += 1;
                    row = row.wrapping_add(ROW_STRIDE);
                }
                (0, count as u32)
            }
        }
        let holder = ((arg + HOLDER) as *const u32).read_unaligned();
        let key_a: u32 =
            lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(KEY_A), 0);
        let (stored_a, _) = scan(holder, key_a);
        ((this + OUT_A) as *mut u32).write_unaligned(stored_a);
        let key_b: u32 =
            lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(KEY_B), 0);
        let (stored_b, ret_b) = scan(holder, key_b);
        ((this + OUT_B) as *mut u32).write_unaligned(stored_b);
        ret_b
    }
});
