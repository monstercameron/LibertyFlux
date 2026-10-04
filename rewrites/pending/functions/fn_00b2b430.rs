// original: 0x00B2B430 range-cursor refresh
/// Shared tail of the range-gate update: notify the watcher, then publish the
/// new cursor into the object.
unsafe fn land_range_tail(obj: u32, cursor: u32) {
    unsafe {
        callee_cdecl!(4, u32, obj);
        (obj as *mut u32).byte_add(0x12A8).write(cursor);
    }
}

/// Refresh an object's range cursor when its window moved.
///
/// Asks the object whether it is active; an idle object keeps its cursor.
/// Otherwise the flag byte decides: a set flag republishes the cursor
/// unconditionally, a clear flag compares the live cursor against the
/// object's window and only samples a fresh probe point when the cursor
/// still lies inside. A probe that answers, whose lead value stays within
/// the unit bound and whose distance from the anchor stays within the
/// scaled limit, leaves the cursor alone; every other outcome republishes
/// it through the shared tail.
export!(cdecl, rw_b2b430(obj: u32, flag: u32) -> u32 {
    const STATUS_SLOT: u32 = 0x128;
    const AUX_OFF: usize = 0x6C;
    const AUX_FLAG_OFF: usize = 0x0E;
    const KEY_OFF: usize = 0xE6E;
    const SPAN_OFF: usize = 0x2C;
    const SPAN_MASK: u32 = 0x7F;
    const MARK_OFF: usize = 0x12A8;
    const ANCHOR_OFF: usize = 0x20;
    const CURSOR_CELL: u32 = 0x0117_35B4;
    const WIDEN_CELL: u32 = 0x0103_FFC4;
    const UNIT_CELL: u32 = 0x00FE_888C;
    unsafe {
        let vt = (obj as *const u32).read();
        let status: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vt.wrapping_add(STATUS_SLOT)) as *const u32).read() as usize,
        );
        if status(obj) & 0xFF == 0 {
            return 0;
        }
        let aux = (obj as *const u32).byte_add(AUX_OFF).read();
        let flagb = flag as u8;
        let gated: u8;
        if aux != 0 && (aux as *const u8).byte_add(AUX_FLAG_OFF).read() != 0 {
            gated = flagb;
        } else {
            let key = (obj as *const u8).byte_add(KEY_OFF).read();
            let probe = callee_cdecl!(2, u32, u32::from(key));
            gated = flagb;
            if probe & 0xFF != 0 && gated == 0 {
                return 0;
            }
        }
        let cursor = global::<u32>(CURSOR_CELL).read();
        if gated != 0 {
            land_range_tail(obj, cursor);
            return 0;
        }
        let mark = (obj as *const u32).byte_add(MARK_OFF).read();
        let span = (u32::from((obj as *const u16).byte_add(SPAN_OFF).read()) & SPAN_MASK)
            .wrapping_add(mark)
            .wrapping_add(global::<u32>(WIDEN_CELL).read());
        if cursor > span || cursor < mark {
            land_range_tail(obj, cursor);
            return 0;
        }
        let mut pt = [0f32; 3];
        let mut scale = 0f32;
        let mut lead = 0f32;
        let ok = callee_cdecl!(
            3,
            u32,
            obj,
            pt.as_mut_ptr() as u32,
            &mut scale as *mut f32 as u32,
            &mut lead as *mut f32 as u32,
            0
        );
        if ok & 0xFF == 0 {
            land_range_tail(obj, cursor);
            return 0;
        }
        let unit = global::<f32>(UNIT_CELL).read();
        if lead > unit {
            land_range_tail(obj, cursor);
            return 0;
        }
        let anchor = (obj as *const u32).byte_add(ANCHOR_OFF).read() as *const f32;
        let dx = pt[0] - anchor.add(12).read();
        let dy = pt[1] - anchor.add(13).read();
        let dz = pt[2] - anchor.add(14).read();
        let dist2 = dx * dx + dy * dy + dz * dz;
        let lim = scale * unit;
        // `jbe` on the distance comparison: unordered (NaN) counts as within.
        if !(dist2 > lim * lim) {
            return 0;
        }
        land_range_tail(obj, cursor);
        0
    }
});
