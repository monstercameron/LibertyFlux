// original: 0x00951db0 slot_table_entry_teardown
/// Tears down one entry of the indexed slot table and marks the slot freed.
///
/// The selector picks one of two 0x818-entry tables and the low word of the
/// index picks the slot; out-of-range indices (the original's range check is
/// a signed comparison, quirks included) and empty slots return at once. A
/// live entry is released according to the type nibble at +0x28: types 2 and
/// 4 run the virtual pre-release/release pair, type 3 additionally unlinks a
/// live side binding, type 1 goes through the registry release, and anything
/// else skips straight to the end. The slot is then cleared with a 0xffff
/// tag. Returns -1 once the slot is cleared, the index on early exits.
export!(cdecl, rw_00951db0(sel: u32, idx: u32) -> u32 {
    unsafe {
        const COUNT: i32 = 0x818;
        const TABLE_A: u32 = 0x0120_8970;
        const TABLE_B: u32 = 0x0121_42D0;
        const ENTRY_STRIDE: i32 = 8;
        const TYPE_OFF: u32 = 0x28;
        const VT_PRE: u32 = 0x98;
        const VT_REL: u32 = 0x30;
        const LINK_OFF: u32 = 0xB30;
        const FLAG_OFF: u32 = 0x26C;
        const FLAG_KEEP: u32 = 0xFFFF_FFFB;
        const FREED_TAG: u16 = 0xFFFF;
        if (idx as i32) < 0 {
            return idx;
        }
        // Signed low-word range check and sign-extending index, exactly like
        // the original (negative lows address memory before the table).
        let e = (idx & 0xFFFF) as u16 as i16 as i32;
        if e >= COUNT {
            return idx;
        }
        let base = relocated(if sel == 1 { TABLE_A } else { TABLE_B });
        let entry = base.wrapping_add(e.wrapping_mul(ENTRY_STRIDE) as u32);
        let obj = *(entry as *const u32);
        if obj == 0 {
            return e as u32;
        }
        let typ = (*((obj.wrapping_add(TYPE_OFF)) as *const u32) >> 6) & 0xF;
        if typ == 2 || typ == 4 || typ == 3 {
            let vt = *(obj as *const u32);
            let pre: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *((vt.wrapping_add(VT_PRE)) as *const u32) as usize,
            );
            pre(obj);
            let vt = *(obj as *const u32);
            let rel: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                *((vt.wrapping_add(VT_REL)) as *const u32) as usize,
            );
            rel(obj, 0);
            if typ == 3 {
                let link_addr = obj.wrapping_add(LINK_OFF);
                let link = *(link_addr as *const u32);
                if link != 0 {
                    callee_thiscall!(3, u32, link, link_addr);
                    let flags = (obj.wrapping_add(FLAG_OFF)) as *mut u32;
                    *flags &= FLAG_KEEP;
                    *(link_addr as *mut u32) = 0;
                }
            }
        } else if typ == 1 {
            callee_cdecl!(4, u32, obj);
        }
        *(entry as *mut u32) = 0;
        *((entry.wrapping_add(4)) as *mut u16) = FREED_TAG;
        0xFFFF_FFFF
    }
});
