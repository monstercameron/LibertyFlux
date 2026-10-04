// original: 0x009509b0 table_slot_rebind_plain
/// Rebinds one entry of the keyed slot table, accepting any nonzero item.
///
/// Twin of the checked variant: same release and re-register sequence, but
/// the new item is published with the idle mark whenever the pointer itself
/// is nonzero, without reading its head word. Returns the item, or 0x5db for
/// an out-of-range key.
export!(cdecl, rw_009509b0(key: u32, item: u32) -> u32 {
    unsafe {
        const MAX_KEY: u32 = 0x5DB;
        const FLAGS: u32 = 0x011F_6958;
        const TABLE: u32 = 0x011F_7110;
        const ENTRY_STRIDE: u32 = 8;
        const KIND_OFF: u32 = 0xA4;
        const IDLE_MARK: u8 = 0;
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
        *flag = 0;
        callee_thiscall!(2, u32, slot);
        if item != 0 {
            *flag = 1;
            *(slot as *mut u32) = item;
            *((slot.wrapping_add(4)) as *mut u8) = IDLE_MARK;
        }
        item
    }
});
