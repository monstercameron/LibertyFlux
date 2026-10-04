// original: 0x00c67850 pool_pick_best
// Pick the best candidate id from the table row selected by `index`.
//
// Each row holds a count followed by up to that many u16 candidate ids. A
// candidate survives four veto checks (one global, three per-object); of the
// survivors the one with the smallest priority word (+0xA8 of its table
// object) wins, ties going to the last one. A survivor is also skipped
// when bit 3 of its flag word (+0x94) is set. Returns the winning id,
// or -1 when the row is empty or nothing survives.
//
// Note: the original keeps its running minimum and winner in two scratch
// words below its incoming ESP (outside the checker's stack window) and
// clobbers its incoming stack slot with the row pointer. A Rust rewrite
// cannot address its incoming stack slot, so the contract disables the
// stack check; the clobbered value is still verified indirectly, because
// the count read back through it drives the loop and the return value.
export!(thiscall, rw_00c67850(obj: u32, index: u32) -> u32 {
    unsafe {
        const ROW_COUNTS: u32 = 0x169e320;
        const ROW_IDS: u32 = 0x169d568;
        const ROW_STRIDE: u32 = 0x46;
        const OBJ_TABLE: u32 = 0x1295cd8;
        const G_MODE: u32 = 0x1295848;
        const G_MODE_OVERRIDE: u32 = 0x1295854;
        const G_TOKEN: u32 = 0x12b4138;

        // Mode select: override wins unless it is -1. The following
        // comparisons always leave the original's flag byte zero, so the
        // mode value itself never affects the result; read it for parity.
        let mode = *global::<u32>(G_MODE);
        let ov = *global::<u32>(G_MODE_OVERRIDE);
        let _mode = if ov != 0xffff_ffff { ov } else { mode };

        let counts = relocated(ROW_COUNTS);
        let count = *(counts.wrapping_add(index.wrapping_mul(4)) as *const i32);
        if count <= 0 {
            return 0xffff_ffff;
        }
        let token = *global::<u32>(G_TOKEN);
        let row = relocated(ROW_IDS).wrapping_add(index.wrapping_mul(ROW_STRIDE));
        let mut min_pri = 0xffff_ffffu32;
        let mut best = 0xffff_ffffu32;
        let mut k = 0i32;
        while k < count {
            let id = *((row.wrapping_add((k as u32).wrapping_mul(2))) as *const u16) as u32;
            let veto1 = callee_cdecl!(1, u32, id, token);
            if veto1 & 0xff != 0 {
                k += 1;
                continue;
            }
            let veto2: u32 = callee_thiscall!(2, u32, obj, id);
            if veto2 & 0xff != 0 {
                k += 1;
                continue;
            }
            let veto3: u32 = callee_thiscall!(3, u32, obj, id);
            if veto3 & 0xff != 0 {
                k += 1;
                continue;
            }
            let veto4: u32 = callee_thiscall!(4, u32, obj, id);
            if veto4 & 0xff != 0 {
                k += 1;
                continue;
            }
            let ent = *(relocated(OBJ_TABLE).wrapping_add(id.wrapping_mul(4)) as *const u32);
            let o = ent as *const u8;
            let flags = *(o.add(0x94) as *const u32);
            if (flags >> 3) & 1 != 0 {
                k += 1;
                continue;
            }
            let pri = *(o.add(0xa8) as *const u32);
            if pri > min_pri {
                k += 1;
                continue;
            }
            min_pri = pri;
            best = id;
            k += 1;
        }
        best
    }
});
