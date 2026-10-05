// original: 0x0062C140 rage::VerletWaterSimulation::vf3

/// Look up one key in the holder's row table, then confirm through a second callee.
///
/// First stage is the shared row scan (key from the lookup callee, rows of
/// 0x30 bytes, `+4` tested before `+0`, unsigned-16-bit count compared as
/// signed). When a row matches, the confirm callee runs (thiscall on the
/// argument object: name string, flags 0) and its zero/non-zero answer is the
/// result; with no match the result is 0 without calling it (stdcall, one
/// stack argument; the object register is unread).
lf_checker_rt::export!(stdcall, rw_0062c140(arg: u32) -> u32 {
    unsafe {
        const HOLDER: u32 = 0x18;
        const ROWS: u32 = 0x08;
        const COUNT: u32 = 0x0c;
        const ROW_STRIDE: u32 = 0x30;
        const KEY_NAME: u32 = 0xFE2360;
        const CONFIRM_NAME: u32 = 0xFE23B4;
        let holder = ((arg + HOLDER) as *const u32).read_unaligned();
        let key: u32 = lf_checker_rt::callee_cdecl!(
            1, u32, lf_checker_rt::relocated(KEY_NAME), 0);
        let count = ((holder + COUNT) as *const u16).read_unaligned() as i32;
        let mut row = ((holder + ROWS) as *const u32).read_unaligned().wrapping_add(0x0c);
        let mut idx: i32 = 0;
        let mut found = false;
        while idx < count {
            if ((row + 4) as *const u32).read_unaligned() == key
                || (row as *const u32).read_unaligned() == key
            {
                found = true;
                break;
            }
            idx += 1;
            row = row.wrapping_add(ROW_STRIDE);
        }
        if !found {
            return 0;
        }
        let ok: u32 =
            lf_checker_rt::callee_thiscall!(2, u32, arg, lf_checker_rt::relocated(CONFIRM_NAME), 0);
        (ok != 0) as u32
    }
});
