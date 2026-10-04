// original: 0x00c570c0 task_query_state_flag (proposed)

/// Read a state flag out of a task object selected by its kind field.
///
/// A null object reads 0. Otherwise bits 6..9 of the dword at `+0x28` select:
/// kind 2 follows the pointer at `+0xf50` and reads 1 when it is non-null and
/// the byte at `+0x219` past it is nonzero, else 0; kind 3 returns the byte at
/// `+0x219` of the object itself; any other kind reads 0. Only the low return
/// byte is defined (the original returns with only `al` set; on the
/// null-object path the upper bytes are the caller's entry `eax`).
///
/// Original: 0x00c570c0 (cdecl, one stack word; return channel `al`).
lf_checker_rt::export!(cdecl, rw_00c570c0(obj: u32) -> u32 {
    unsafe {
        const KIND_WORD: u32 = 0x28;
        const KIND_SHIFT: u32 = 6;
        const LINK: u32 = 0xf50;
        const FLAG: u32 = 0x219;
        if obj == 0 {
            return 0;
        }
        let kind = (((obj + KIND_WORD) as *const u32).read_unaligned() >> KIND_SHIFT) & 0xf;
        if kind == 2 {
            let p = ((obj + LINK) as *const u32).read_unaligned();
            if p != 0 && ((p + FLAG) as *const u8).read() != 0 {
                return 1;
            }
            return 0;
        }
        if kind == 3 {
            return ((obj + FLAG) as *const u8).read() as u32;
        }
        0
    }
});

