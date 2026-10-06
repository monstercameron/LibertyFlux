// original: 0x0065BA20 rage::EnviromentDomeLighting::EnviromentDomeVarCache::vf0 (symbols)

/// Resolve two keys in the holder's row table and store their 1-based slots.
///
/// Two stages of the shared row scan (key from the lookup callee, cdecl:
/// name string, flags 0; rows of 0x30 bytes at the table `+0x08`, word at
/// `+4` tested before word at `+0`, unsigned-16-bit count at `+0x0c`
/// compared as signed but never negative; key equality only, so signedness
/// is unobservable). The first slot (match index + 1, else 0) goes to
/// `this + 4`, the second to `this + 8`. Returns the second stage's slot,
/// or the row count when the second stage finds nothing (thiscall, one
/// stack argument).
lf_checker_rt::export!(thiscall, rw_0065ba20(this: u32, arg: u32) -> u32 {
    unsafe {
        const HOLDER: u32 = 0x18;
        const ROWS: u32 = 0x08;
        const COUNT: u32 = 0x0c;
        const ROW_STRIDE: u32 = 0x30;
        const KEY1_NAME: u32 = 0xFE2EAC;
        const KEY2_NAME: u32 = 0xFE2F08;
        unsafe fn scan(holder: u32, key: u32) -> (i32, i32, bool) {
            unsafe {
                let count = ((holder + COUNT) as *const u16).read_unaligned() as i32;
                let mut row =
                    ((holder + ROWS) as *const u32).read_unaligned().wrapping_add(0x0c);
                let mut idx: i32 = 0;
                while idx < count {
                    if ((row + 4) as *const u32).read_unaligned() == key
                        || (row as *const u32).read_unaligned() == key
                    {
                        return (count, idx + 1, true);
                    }
                    idx += 1;
                    row = row.wrapping_add(ROW_STRIDE);
                }
                (count, 0, false)
            }
        }
        let holder = ((arg + HOLDER) as *const u32).read_unaligned();
        let key1: u32 = lf_checker_rt::callee_cdecl!(
            1, u32, lf_checker_rt::relocated(KEY1_NAME), 0);
        let (_, slot1, _) = scan(holder, key1);
        ((this + 4) as *mut u32).write_unaligned(slot1 as u32);
        let holder2 = ((arg + HOLDER) as *const u32).read_unaligned();
        let key2: u32 = lf_checker_rt::callee_cdecl!(
            2, u32, lf_checker_rt::relocated(KEY2_NAME), 0);
        let (count2, slot2, found2) = scan(holder2, key2);
        if found2 {
            ((this + 8) as *mut u32).write_unaligned(slot2 as u32);
            slot2 as u32
        } else {
            ((this + 8) as *mut u32).write_unaligned(0);
            count2 as u32
        }
    }
});
