// original: 0x00a98a10 filemem_sweep_stale_slots

/// Sweep the slot list twice, retiring entries the check callee rejects.
///
/// `this` points to an object whose word at `+0x8ec50` heads a chain of
/// slot nodes linked through their first word. The first pass asks the
/// check callee about `this` at every node and returns at once when its
/// low byte is zero (an exact zero check); a node whose priority float at
/// `+0x78` is strictly above the global limit and whose target (at
/// `+0x68`) carries flag bit 0x400000 at its `+0x24` is retired through
/// the detach and destroy callees. The second pass repeats the walk
/// without the priority test. The float comparison is ordered: NaN never
/// retires.
///
/// Original: 0x00A98A10 (thiscall, no stack arguments; three direct callees).
lf_checker_rt::export!(thiscall, rw_00a98a10(this: u32) -> u32 {
    unsafe {
        /// Head of the slot-node chain, from the object base.
        const LIST_OFF: u32 = 0x8ec50;
        /// Next-node link / target object / priority float, from a node.
        const NODE_NEXT: u32 = 0x00;
        const NODE_TARGET: u32 = 0x68;
        const NODE_PRIO: u32 = 0x78;
        /// Flag bit (at target +0x24) a retiring node must carry.
        const RETIRE_FLAG: u32 = 0x00040000;
        /// Global priority limit (file VA).
        const LIMIT: u32 = 0x00e9cae0;
        const CHECK: u32 = 1;
        const DETACH: u32 = 2;
        const DESTROY: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        let limit = f32::from_bits(rd32(lf_checker_rt::relocated(LIMIT)));
        // First pass: priority test included.
        let mut node = rd32(this.wrapping_add(LIST_OFF));
        while node != 0 {
            let next = rd32(node.wrapping_add(NODE_NEXT));
            let ok: u32 = lf_checker_rt::callee_thiscall!(CHECK, u32, this);
            if (ok as u8) == 0 {
                return 0;
            }
            if rdf(node.wrapping_add(NODE_PRIO)) > limit {
                let target = rd32(node.wrapping_add(NODE_TARGET));
                if rd32(target.wrapping_add(0x24)) & RETIRE_FLAG != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(DETACH, u32, this, target);
                    let _: u32 = lf_checker_rt::callee_thiscall!(DESTROY, u32, node);
                }
            }
            node = next;
        }
        // Second pass: no priority test.
        let mut node = rd32(this.wrapping_add(LIST_OFF));
        while node != 0 {
            let next = rd32(node.wrapping_add(NODE_NEXT));
            let ok: u32 = lf_checker_rt::callee_thiscall!(CHECK, u32, this);
            if (ok as u8) == 0 {
                return 0;
            }
            let target = rd32(node.wrapping_add(NODE_TARGET));
            if rd32(target.wrapping_add(0x24)) & RETIRE_FLAG != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(DETACH, u32, this, target);
                let _: u32 = lf_checker_rt::callee_thiscall!(DESTROY, u32, node);
            }
            node = next;
        }
        0
    }
});
