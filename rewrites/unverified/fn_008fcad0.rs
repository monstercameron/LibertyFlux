// original: 0x008fcad0 input_event_append (proposed)

/// Append one input event to the indexed event table.
///
/// Seven stack words (`p1` is never read; the rest are described below) and
/// the global event index select the table slot. When the slot is empty a
/// block is allocated (cdecl, 0x51C bytes): a null answer records the empty
/// slot, advances the index and returns the old index, while a live block
/// is constructed in place (thiscall: the block with `p2..p7` as six stack
/// words), stored to the slot, and likewise advances and returns. When the
/// slot already holds an event it is filled in place instead: `p2` at
/// `+8`, the two dwords under `p3` at `+0x18`/`+0x1C`, the two under `p4`
/// at `+0x20`/`+0x24`, `p5` at `+0x30`, `p6` at `+0x44` and the low byte of
/// `p7` at `+0x58`; the index advances and the old index is returned.
///
/// Cdecl: seven stack words, caller cleans.
lf_checker_rt::export!(cdecl, rw_008fcad0(_p1: u32, p2: u32, p3: u32, p4: u32, p5: u32, p6: u32, p7: u32) -> u32 {
    unsafe {
        const C_ALLOC: u32 = 1;
        const C_CTOR: u32 = 2;
        const G_INDEX: u32 = 0x118e920;
        const G_TABLE: u32 = 0x118e7f8;
        const ALLOC_BYTES: u32 = 0x51c;
        let idx = (lf_checker_rt::global::<u32>(G_INDEX)).read_unaligned();
        let tab = lf_checker_rt::global::<u32>(G_TABLE);
        let cur = tab.add(idx as usize).read_unaligned();
        if cur == 0 {
            let mem: u32 = lf_checker_rt::callee_cdecl!(C_ALLOC, u32, ALLOC_BYTES);
            if mem == 0 {
                tab.add(idx as usize).write_unaligned(0);
                (lf_checker_rt::global::<u32>(G_INDEX)).write_unaligned(idx.wrapping_add(1));
                return idx;
            }
            let built: u32 =
                lf_checker_rt::callee_thiscall!(C_CTOR, u32, mem, p2, p3, p4, p5, p6, p7);
            tab.add(idx as usize).write_unaligned(built);
            (lf_checker_rt::global::<u32>(G_INDEX)).write_unaligned(idx.wrapping_add(1));
            return idx;
        }
        ((cur + 0x08) as *mut u32).write_unaligned(p2);
        ((cur + 0x18) as *mut u32).write_unaligned((p3 as *const u32).read_unaligned());
        ((cur + 0x1c) as *mut u32).write_unaligned(((p3 + 4) as *const u32).read_unaligned());
        ((cur + 0x20) as *mut u32).write_unaligned((p4 as *const u32).read_unaligned());
        ((cur + 0x24) as *mut u32).write_unaligned(((p4 + 4) as *const u32).read_unaligned());
        ((cur + 0x30) as *mut u32).write_unaligned(p5);
        ((cur + 0x44) as *mut u32).write_unaligned(p6);
        ((cur + 0x58) as *mut u8).write((p7 & 0xff) as u8);
        (lf_checker_rt::global::<u32>(G_INDEX)).write_unaligned(idx.wrapping_add(1));
        idx
    }
});
