// original: 0x00950910 table_slot_rebind_checked
/// Rebinds one entry of the keyed slot table, keeping only live items.
///
/// The low word of the key indexes a flag array and an 8-byte-entry table.
/// When the slot is active, a stale binding whose kind tag disagrees with the
/// key in the full word but matches it in the low word (or an unflagged
/// entry) is released first. The slot is then cleared and re-registered, and
/// the new item is published with the live mark only when it and its head
/// word are both nonzero. Returns the item, or 0x5db for an out-of-range key
/// (the bound the original leaves in eax on that path).
export!(cdecl, rw_00950910(key: u32, item: u32) -> u32 {
    unsafe {
        const MAX_KEY: u32 = 0x5DB;
        const FLAGS: u32 = 0x011F_6958;
        const TABLE: u32 = 0x011F_7110;
        const ENTRY_STRIDE: u32 = 8;
        const KIND_OFF: u32 = 0xA4;
        const LIVE_MARK: u8 = 1;
        let index = key & 0xFFFF;
        if index > MAX_KEY {
            return MAX_KEY;
        }
        let flag = (relocated(FLAGS).wrapping_add(index)) as *mut u8;
        let slot = relocated(TABLE).wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
        if *flag != 0 {
            let entry = *(slot as *const u32);
            if entry != 0 {
                if *((slot.wrapping_add(4)) as *const u8) == 0 {
                    callee_thiscall!(1, u32, entry, 0);
                } else {
                    let inner = *(entry as *const u32);
                    if inner != 0 {
                        let kind = *((inner.wrapping_add(KIND_OFF)) as *const u32);
                        if kind != key && kind & 0xFFFF == index {
                            callee_thiscall!(1, u32, inner, 0);
                        }
                    }
                }
            }
        }
        // The abort call for index >= 0x5dc is dead: the early return above
        // leaves index <= 0x5db on every path that reaches here.
        *flag = 0;
        callee_thiscall!(2, u32, slot);
        if item != 0 && *(item as *const u32) != 0 {
            *flag = 1;
            *(slot as *mut u32) = item;
            *((slot.wrapping_add(4)) as *mut u8) = LIVE_MARK;
        }
        item
    }
});
