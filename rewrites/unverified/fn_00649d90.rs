// original: 0x00649d90 ptx_effect_index_scan (proposed)

/// Resolve the four fixed effect keys to row indexes and store them at
/// `this + 0x94`: each key is resolved to an id through the name callee,
/// then the id is scanned for in the group table at `arg + 0x18` (row base
/// at `+0x8`, signed 16-bit row count at `+0xc`, 0x30-byte rows compared at
/// `+0x0` and `+0x4` past a 0xc header). The stored value is the 1-based
/// index of the first matching row, or zero when the count is not positive
/// or no row matches. Returns the last resolved id. Thiscall, one stack
/// argument. The key words are the immediates the original pushes, kept
/// literally so the outgoing calls compare.
lf_checker_rt::export!(thiscall, rw_00649d90(this: u32, arg: u32) -> u32 {
    unsafe {
        const KEYS: [u32; 4] = [0x00f9_9d68, 0x00f9_9d74, 0x00f9_9d20, 0x00f9_9d2c];
        const GROUP_TABLE: u32 = 0x18;
        const ROW_BASE: u32 = 0x08;
        const ROW_COUNT: u32 = 0x0c;
        const ROW_HEADER: u32 = 0x0c;
        const ROW_STRIDE: u32 = 0x30;
        const OUT_BASE: u32 = 0x94;
        const NAME_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let mut last_id = 0u32;
        let mut out = this.wrapping_add(OUT_BASE);
        for key in KEYS {
            let id = lf_checker_rt::callee_cdecl!(NAME_CALLEE, u32, key, 0);
            last_id = id;
            let group = read_word(arg.wrapping_add(GROUP_TABLE));
            let count =
                ((group.wrapping_add(ROW_COUNT)) as *const u16).read_unaligned()
                    as i16 as i32;
            let mut found = 0u32;
            if count > 0 {
                let mut row =
                    read_word(group.wrapping_add(ROW_BASE)).wrapping_add(ROW_HEADER);
                let mut index = 0i32;
                while index < count {
                    if read_word(row.wrapping_add(4)) == id
                        || read_word(row) == id
                    {
                        found = (index as u32).wrapping_add(1);
                        break;
                    }
                    index += 1;
                    row = row.wrapping_add(ROW_STRIDE);
                }
            }
            (out as *mut u32).write_unaligned(found);
            out = out.wrapping_add(4);
        }
        last_id
    }
});
