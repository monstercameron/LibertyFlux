// original: 0x00b29cc0 advance_slot_cursor
/// Advance a slot cursor by a distance and refresh its object.
///
/// Finds the slot whose id matches (scanning slots 0..23 for the first
/// valid entry equal to the id; an unmatched id walks the spill slot past
/// the table), then moves the slot cursor in 0x20 steps toward the
/// distance: growing while it is positive, shrinking while negative, each
/// step consuming the segment length between the old and new positions.
/// Stores the position value as a float, and when the slot's refresh flag
/// is set copies the object's twelve-float block into a scratch frame,
/// runs the object update on the record at the final cursor, and notifies
/// each item in turn. Returns the object's address on the refresh path,
/// otherwise the top bit of the position value.
export!(cdecl, rw_00b29cc0(id: u32, dist_bits: u32) -> u32 {
    unsafe {
        const T_IDS: u32 = 0x01657650;
        const T_BASES: u32 = 0x016576B0;
        const T_CURSORS: u32 = 0x01657710;
        const T_LIMITS: u32 = 0x01657770;
        const T_PROGRESS: u32 = 0x016577D0;
        const T_VALID: u32 = 0x01657890;
        const T_REFRESH: u32 = 0x016578D8;
        const T_TRIPLES: u32 = 0x0165FBA0;
        const STEP: u32 = 0x20;
        const ITEM_STRIDE: u32 = 0x170;
        let update: extern "cdecl" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(7) as usize);
        let notify_item: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(8) as usize);

        let mut slot = 24u32;
        for i in 0..24u32 {
            let valid = *((relocated(T_VALID).wrapping_add(i)) as *const u8);
            let entry =
                *((relocated(T_IDS).wrapping_add(i.wrapping_mul(4))) as *const u32);
            if valid != 0 && entry == id {
                slot = i;
                break;
            }
        }
        let base =
            *((relocated(T_BASES).wrapping_add(slot.wrapping_mul(4))) as *const u32);
        let cursor_cell =
            (relocated(T_CURSORS).wrapping_add(slot.wrapping_mul(4))) as *mut u32;
        let mut cursor = *cursor_cell;
        let limit =
            *((relocated(T_LIMITS).wrapping_add(slot.wrapping_mul(4))) as *const u32);
        let mut d = f32::from_bits(dist_bits);
        loop {
            if d > 0.0 {
                if cursor >= limit.wrapping_sub(STEP) {
                    break;
                }
                let old = cursor;
                cursor = cursor.wrapping_add(STEP);
                *cursor_cell = cursor;
                let ax = base.wrapping_add(old);
                let di = base.wrapping_add(cursor);
                let dx = *(ax.wrapping_add(0x14) as *const f32)
                    - *(di.wrapping_add(0x14) as *const f32);
                let dy = *(ax.wrapping_add(0x18) as *const f32)
                    - *(di.wrapping_add(0x18) as *const f32);
                d -= (dx * dx + dy * dy).sqrt();
                // Grow loops back only while d is still above zero (the
                // original's jb tests 0-vs-d flags: CF iff d > 0 or NaN).
                if !(d <= 0.0) {
                    continue;
                }
                break;
            } else {
                if cursor <= STEP {
                    break;
                }
                let old = cursor;
                cursor = cursor.wrapping_sub(STEP);
                *cursor_cell = cursor;
                let ax = base.wrapping_add(old);
                let di = base.wrapping_add(cursor);
                let dx = *(ax.wrapping_add(0x14) as *const f32)
                    - *(di.wrapping_add(0x14) as *const f32);
                let dy = *(ax.wrapping_add(0x18) as *const f32)
                    - *(di.wrapping_add(0x18) as *const f32);
                d += (dx * dx + dy * dy).sqrt();
                if !(d >= 0.0) {
                    continue;
                }
                break;
            }
        }
        let edi = base.wrapping_add(cursor);
        let pos = *(edi as *const u32);
        *(relocated(T_PROGRESS).wrapping_add(slot.wrapping_mul(4)) as *mut f32) =
            (pos as f64) as f32;
        if *((relocated(T_REFRESH).wrapping_add(slot)) as *const u8) == 0 {
            return pos >> 31;
        }
        let obj =
            *((relocated(T_IDS).wrapping_add(slot.wrapping_mul(4))) as *const u32);
        let src = *((obj.wrapping_add(0x20)) as *const u32);
        let mut buf = [0u32; 15];
        buf[0] = *(src as *const u32);
        buf[1] = *(src.wrapping_add(4) as *const u32);
        buf[2] = *(src.wrapping_add(8) as *const u32);
        buf[4] = *(src.wrapping_add(0x10) as *const u32);
        buf[5] = *(src.wrapping_add(0x14) as *const u32);
        buf[6] = *(src.wrapping_add(0x18) as *const u32);
        buf[8] = *(src.wrapping_add(0x20) as *const u32);
        buf[9] = *(src.wrapping_add(0x24) as *const u32);
        buf[10] = *(src.wrapping_add(0x28) as *const u32);
        buf[12] = *(src.wrapping_add(0x30) as *const u32);
        buf[13] = *(src.wrapping_add(0x34) as *const u32);
        buf[14] = *(src.wrapping_add(0x38) as *const u32);
        let rec2 = base.wrapping_add(*cursor_cell);
        let arg4 = relocated(T_TRIPLES).wrapping_add(slot.wrapping_mul(16));
        update(obj, rec2, 0, arg4);
        let items_base = *((obj.wrapping_add(0xF80)) as *const u32);
        let extra = *((obj.wrapping_add(0x20)) as *const u32);
        let mut i = 0u32;
        loop {
            // Signed bounds, as the original's jl/jge on the count dword.
            let count = *((obj.wrapping_add(0xF84)) as *const i32);
            if !((i as i32) < count) {
                break;
            }
            let this2 = if (i as i32) < count {
                items_base.wrapping_add(i.wrapping_mul(ITEM_STRIDE))
            } else {
                0
            };
            notify_item(this2, buf.as_mut_ptr() as u32, extra);
            i = i.wrapping_add(1);
        }
        obj
    }
});
