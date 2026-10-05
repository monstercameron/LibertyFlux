// original: 0x00878E00 rage::crmtComposerOptimized::vf31

/// Run a gate over the pending list, freeing accepted nodes.
///
/// `this` points to a composer object with a staging slot at `+0x8c`.
/// When staging is empty the pool-sync virtual (slot `+0x74`) runs, the
/// pending list at `+0x80` moves to staging (`+0x80`/`+0x84` cleared),
/// and an empty move accepts at once. Then each staged node runs the
/// gate virtual (slot `+0x84`) with (`this`, node, `a0`): a zero low
/// byte rejects the whole poll at once, otherwise the node advances
/// past staging onto the free list at `+0x88`. Draining staging accepts;
/// either way the counters at `+0x90` and `+0x94` are cleared on the
/// accepting paths.
///
/// Original: 0x00878E00 (thiscall, one stack argument; low byte of the
/// return only).
lf_checker_rt::export!(thiscall, rw_00878E00(this: u32, a0: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x80;
        const TAIL_OFF: u32 = 0x84;
        const STAGE_OFF: u32 = 0x8c;
        const FREE_OFF: u32 = 0x88;
        const COUNT0_OFF: u32 = 0x90;
        const COUNT1_OFF: u32 = 0x94;
        const NEXT_OFF: u32 = 4;
        const SYNC_VT_SLOT: u32 = 0x74;
        const GATE_VT_SLOT: u32 = 0x84;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let vtable = rd32(this);
        if rd32(this.wrapping_add(STAGE_OFF)) == 0 {
            let sync: extern "thiscall" fn(u32) -> u32 = unsafe {
                core::mem::transmute(rd32(vtable.wrapping_add(SYNC_VT_SLOT)) as usize)
            };
            sync(this);
            let moved = rd32(this.wrapping_add(HEAD_OFF));
            wr32(this.wrapping_add(STAGE_OFF), moved);
            wr32(this.wrapping_add(HEAD_OFF), 0);
            wr32(this.wrapping_add(TAIL_OFF), 0);
            if moved == 0 {
                wr32(this.wrapping_add(COUNT1_OFF), 0);
                wr32(this.wrapping_add(COUNT0_OFF), 0);
                return 1;
            }
        }
        let gate: extern "thiscall" fn(u32, u32, u32) -> u32 = unsafe {
            core::mem::transmute(rd32(vtable.wrapping_add(GATE_VT_SLOT)) as usize)
        };
        loop {
            let node = rd32(this.wrapping_add(STAGE_OFF));
            if (gate(this, node, a0) as u8) == 0 {
                return 0;
            }
            wr32(this.wrapping_add(STAGE_OFF), rd32(node.wrapping_add(NEXT_OFF)));
            let free = rd32(this.wrapping_add(FREE_OFF));
            wr32(node.wrapping_add(NEXT_OFF), free);
            wr32(this.wrapping_add(FREE_OFF), node);
            if rd32(this.wrapping_add(STAGE_OFF)) == 0 {
                wr32(this.wrapping_add(COUNT1_OFF), 0);
                wr32(this.wrapping_add(COUNT0_OFF), 0);
                return 1;
            }
        }
    }
});
