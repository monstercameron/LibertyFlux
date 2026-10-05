// original: 0x0062C320 rage::VerletWaterPerturbation::VerletWaterPerturbationVarCache::vf0

/// Cache the row index of one key: the shared scan, storing index + 1.
///
/// Same row scan as the `vf3` family (key from the lookup callee, 0x30-byte
/// rows, `+4` before `+0`, unsigned-16-bit count compared as signed), but the
/// 1-based position of the first match is stored at `OUT` (`+4` of `this`),
/// or 0 when no row matches. Returns the stored value on a match and the
/// scan count otherwise, as the original leaves it (thiscall, one argument).
lf_checker_rt::export!(thiscall, rw_0062c320(this: u32, arg: u32) -> u32 {
    unsafe {
        const HOLDER: u32 = 0x18;
        const ROWS: u32 = 0x08;
        const COUNT: u32 = 0x0c;
        const ROW_STRIDE: u32 = 0x30;
        const OUT: u32 = 0x04;
        const KEY_NAME: u32 = 0xFE23C8;
        let holder = ((arg + HOLDER) as *const u32).read_unaligned();
        let key: u32 = lf_checker_rt::callee_cdecl!(
            1, u32, lf_checker_rt::relocated(KEY_NAME), 0);
        let count = ((holder + COUNT) as *const u16).read_unaligned() as i32;
        let mut row = ((holder + ROWS) as *const u32).read_unaligned().wrapping_add(0x0c);
        let mut idx: i32 = 0;
        while idx < count {
            if ((row + 4) as *const u32).read_unaligned() == key
                || (row as *const u32).read_unaligned() == key
            {
                let pos = (idx + 1) as u32;
                ((this + OUT) as *mut u32).write_unaligned(pos);
                return pos;
            }
            idx += 1;
            row = row.wrapping_add(ROW_STRIDE);
        }
        ((this + OUT) as *mut u32).write_unaligned(0);
        count as u32
    }
});
