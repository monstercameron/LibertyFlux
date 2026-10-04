// original: 0x00e16542 slot_state_update
/// Update one slot's flag and mode bytes, returning the previous flag state.
///
/// Slots live in record blocks reached through the module's pointer table:
/// the block is `table[idx >> 5]` and the record starts `(idx & 0x1F) << 6`
/// bytes in. The flag byte sits 4 past the record start, the mode byte
/// `0x24` past it. `mode` selects the update: `0x4000` sets the flag and
/// keeps only the mode byte's top bit; `0x8000` clears the flag; `0x10000`
/// and `0x20000` set the flag and select predicate bit 1; `0x40000` sets the
/// flag and selects predicate bit 0; anything else writes nothing. The return
/// value depends only on the entry bytes: `0x8000` when the flag bit was
/// clear, otherwise `0x4000` when the mode byte's low seven bits were all
/// clear and `0x10000` when any was set.
export!(cdecl, rw_00e16542(idx: u32, mode: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x17AC2B8;
        const FLAG_OFF: usize = 4;
        const MODE_OFF: usize = 0x24;
        const FLAG_BIT: u8 = 0x80;
        const MODE_SET_BITS: u32 = 0x4000;
        const MODE_CLEAR: u32 = 0x8000;
        const MODE_PRED1_A: u32 = 0x10000;
        const MODE_PRED1_B: u32 = 0x20000;
        const MODE_PRED0: u32 = 0x40000;
        const RET_WAS_CLEAR: u32 = 0x8000;
        const RET_LOW7_CLEAR: u32 = 0x4000;
        const RET_LOW7_SET: u32 = 0x10000;
        let table = global::<u32>(TABLE);
        let base = *table.add((idx >> 5) as usize) as *mut u8;
        let off = ((idx & 0x1F) << 6) as usize;
        let flag = base.add(off + FLAG_OFF);
        let modeb = base.add(off + MODE_OFF);
        let was_set = *flag & FLAG_BIT != 0;
        let entry_mode = *modeb;
        match mode {
            MODE_SET_BITS => {
                *flag |= FLAG_BIT;
                *modeb &= FLAG_BIT;
            }
            MODE_CLEAR => {
                *flag &= !FLAG_BIT;
            }
            MODE_PRED1_A | MODE_PRED1_B => {
                *flag |= FLAG_BIT;
                *modeb = (*modeb & 0x82) | 2;
            }
            MODE_PRED0 => {
                *flag |= FLAG_BIT;
                *modeb = (*modeb & 0x81) | 1;
            }
            _ => {}
        }
        if !was_set {
            return RET_WAS_CLEAR;
        }
        if entry_mode & 0x7F == 0 {
            RET_LOW7_CLEAR
        } else {
            RET_LOW7_SET
        }
    }
});
