// original: 0x00c6c550 stream_detach_bank (proposed)

/// Detach id from the four banks, stamp its object, and charge the cost.
///
/// Removes `id` from the id arrays at `this+0`, `+0x100`, `+0x200` and
/// `+0x300` through the shared remover, stamps the object's word at
/// +0xA8 with the current tick, then settles the cost: a seed combined
/// from the two cost globals is handed to the combiner through three
/// frame-resident out-slots (one preloaded with the seed, two scratch)
/// and the two settled words are subtracted from the accumulator at
/// +0x70C. The out-slot addresses differ per side by construction, so
/// the stack check is off for this proof; the settled values still
/// reach the heap and the preloaded seed is captured by snapshot.
///
/// Original: thiscall with one stack word, five call sites, reads six
/// globals.
lf_checker_rt::export!(thiscall, rw_00c6c550(this: u32, id: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0129_5CD8;
        const TICK_NOW: u32 = 0x0117_35B4;
        const COST_K: u32 = 0x0103_2F58;
        const COST_TAB: u32 = 0x0130_53A8;
        const COST_C: u32 = 0x0103_DBBC;
        const LOCK: u32 = 0x012B_4138;
        const STAMP_OFF: u32 = 0xA8;
        const ACC_OFF: u32 = 0x70C;
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

        for off in [0u32, 0x100, 0x200, 0x300] {
            lf_checker_rt::callee_thiscall!(REMOVE, u32, this.wrapping_add(off), id);
        }
        let table = lf_checker_rt::relocated(TABLE);
        let obj = rd32(table.wrapping_add(id.wrapping_mul(4)));
        wr32(
            obj.wrapping_add(STAMP_OFF),
            rd32(lf_checker_rt::relocated(TICK_NOW)),
        );
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
        let acc = rd32(this.wrapping_add(ACC_OFF)).wrapping_sub(sum);
        wr32(this.wrapping_add(ACC_OFF), acc);
        0
    }
});
