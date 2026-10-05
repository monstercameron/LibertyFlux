// original: 0x00c694e0 stream_unregister_entry (proposed)

/// Unregister id from the three groups, clear its flag, and charge cost.
///
/// Removes `id` from the id arrays at `this+0x404`, `+0x504`, `+0x604`
/// through the shared remover, stamps the object's word at +0x94 with
/// the current tick, and when the object's low flag bit at +0x158 is
/// set clears it and drops the live count at +0x704. The cost settles
/// exactly like the sibling detach routine (seeded combiner through
/// three frame-resident out-slots) and is subtracted from the budget
/// at +0x708, clamped at zero. Same proof narrowing: stack check off,
/// settled values verified through the heap, seed through a snapshot.
///
/// Original: thiscall with one stack word, four call sites, reads six
/// globals.
lf_checker_rt::export!(thiscall, rw_00c694e0(this: u32, id: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0129_5CD8;
        const TICK_NOW: u32 = 0x0117_35B4;
        const COST_K: u32 = 0x0103_2F58;
        const COST_TAB: u32 = 0x0130_53A8;
        const COST_C: u32 = 0x0103_DBBC;
        const LOCK: u32 = 0x012B_4138;
        const STAMP_OFF: u32 = 0x94;
        const FLAG_OFF: u32 = 0x158;
        const LIVE_OFF: u32 = 0x704;
        const BUDGET_OFF: u32 = 0x708;
        const REMOVE: u32 = 1;
        const COMBINE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        for off in [0x404u32, 0x504, 0x604] {
            lf_checker_rt::callee_thiscall!(REMOVE, u32, this.wrapping_add(off), id);
        }
        let table = lf_checker_rt::relocated(TABLE);
        let obj = rd32(table.wrapping_add(id.wrapping_mul(4)));
        wr32(
            obj.wrapping_add(STAMP_OFF),
            rd32(lf_checker_rt::relocated(TICK_NOW)),
        );
        let flags = rd32(obj.wrapping_add(FLAG_OFF));
        if flags & 1 != 0 {
            wr32(obj.wrapping_add(FLAG_OFF), flags & !1);
            let live = rd32(this.wrapping_add(LIVE_OFF)).wrapping_sub(1);
            wr32(this.wrapping_add(LIVE_OFF), live);
        }
        let k = rd32(lf_checker_rt::relocated(COST_K));
        let tab = rd32(
            lf_checker_rt::relocated(COST_TAB).wrapping_add(k.wrapping_mul(0x64)),
        );
        let seed = tab.wrapping_add(rd32(lf_checker_rt::relocated(COST_C)));
        let lock = rd32(lf_checker_rt::relocated(LOCK));
        let mut slot_a = seed;
        let mut slot_b = 0u32;
        let mut slot_c = 0u32;
        lf_checker_rt::callee_cdecl!(
            COMBINE,
            u32,
            id,
            lock,
            &mut slot_c as *mut u32 as u32,
            &mut slot_b as *mut u32 as u32,
            &mut slot_a as *mut u32 as u32,
            1
        );
        let sum = slot_c.wrapping_add(slot_b);
        let budget = rd32(this.wrapping_add(BUDGET_OFF)).wrapping_sub(sum);
        wr32(
            this.wrapping_add(BUDGET_OFF),
            if (budget as i32) < 0 { 0 } else { budget },
        );
        0
    }
});
