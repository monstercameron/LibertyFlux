// original: 0x00a9c090 filemem_collect_ready

/// Collect ready items: poll the live set, then sweep the waiting chain.
///
/// `this` points to the collector, `a1` is passed on to the sweep callee.
/// The handle at `+0x68` must be non-null with a non-null object at
/// `+0x34` whose status word is nonzero, else 0. The resolve callee
/// (handle vtable slot `+0xa0`) names the live set; a null answer skips to
/// the sweep. Otherwise the describe callee names its descriptor, and the
/// poll loop runs the descriptor's count byte at `+0x1f2` (zero skips the
/// loop): index `esi` runs from 0 while `esi` stays below the count (both
/// non-negative, so the original's signed bound matches an unsigned one);
/// every index but the one equal to the skip byte (table at descriptor
/// `+0xd4` indexed by the word at `this+0x8`, byte at `+0xc`) is offered
/// to the poll callee with the live set, and a zero low byte aborts to
/// the sweep. A completed loop runs the finish callee (slot `+0x98`) and
/// answers 0. The sweep walks the chain at `this+0x10` (an empty chain
/// answers 0 at once), offering each node with `this` and `a1` to the
/// sweep callee and counting nodes whose byte at `+0x14` is set; a zero
/// count answers 0, otherwise the flush callee runs on the live set when
/// it is non-null, the commit callee runs on `this`, and the count is
/// answered.
///
/// Original: 0x00A9C090 (thiscall, one stack word; two indirect callees,
/// five direct callees).
lf_checker_rt::export!(thiscall, rw_00a9c090(this: u32, a1: u32) -> u32 {
    unsafe {
        /// Handle, from the collector; object and status, from a handle.
        const HANDLE_OFF: u32 = 0x68;
        const HANDLE_OBJ: u32 = 0x34;
        /// Resolve/finish slots in the handle vtable.
        const VT_RESOLVE: u32 = 0xa0;
        const VT_FINISH: u32 = 0x98;
        /// Table index, from the collector; table/count/skip, from a descriptor.
        const INDEX_OFF: u32 = 0x8;
        const DESC_TABLE: u32 = 0xd4;
        const DESC_COUNT: u32 = 0x1f2;
        const SKIP_OFF: u32 = 0xc;
        /// Sweep chain, from the collector; next/count-byte, from a node.
        const CHAIN_OFF: u32 = 0x10;
        const NODE_NEXT: u32 = 0x0;
        const NODE_FLAG: u32 = 0x14;
        const DESCRIBE: u32 = 2;
        const POLL: u32 = 4;
        const SWEEP: u32 = 5;
        const FLUSH: u32 = 7;
        const COMMIT: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let h = rd32(this.wrapping_add(HANDLE_OFF));
        if h == 0 {
            return 0;
        }
        let obj = rd32(h.wrapping_add(HANDLE_OBJ));
        if obj == 0 || rd32(obj) == 0 {
            return 0;
        }
        let rslot = rd32(rd32(h).wrapping_add(VT_RESOLVE));
        let resolve: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rslot as usize);
        let live = resolve(h);
        if live != 0 {
            let desc: u32 = lf_checker_rt::callee_thiscall!(DESCRIBE, u32, live);
            let edx = rd32(this.wrapping_add(INDEX_OFF));
            let count = rd8(desc.wrapping_add(DESC_COUNT));
            let tab = rd32(desc.wrapping_add(DESC_TABLE));
            let skip = rd8(rd32(tab.wrapping_add(edx.wrapping_mul(4))).wrapping_add(SKIP_OFF));
            if count == 0 {
                let fslot = rd32(rd32(h).wrapping_add(VT_FINISH));
                let finish: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(fslot as usize);
                let _: u32 = finish(h);
                return 0;
            }
            let mut to_sweep = false;
            let mut i: u32 = 0;
            while i < count as u32 {
                if i != skip as u32 {
                    let r: u32 = lf_checker_rt::callee_thiscall!(POLL, u32, live, i);
                    if (r as u8) == 0 {
                        to_sweep = true;
                        break;
                    }
                }
                i = i.wrapping_add(1);
            }
            if !to_sweep {
                let fslot = rd32(rd32(h).wrapping_add(VT_FINISH));
                let finish: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(fslot as usize);
                let _: u32 = finish(h);
                return 0;
            }
        }
        // Sweep phase.
        let mut node = rd32(this.wrapping_add(CHAIN_OFF));
        if node == 0 {
            return 0;
        }
        let mut ready: u32 = 0;
        loop {
            let _: u32 = lf_checker_rt::callee_thiscall!(SWEEP, u32, node, this, a1);
            if rd8(node.wrapping_add(NODE_FLAG)) != 0 {
                ready = ready.wrapping_add(1);
            }
            node = rd32(node.wrapping_add(NODE_NEXT));
            if node == 0 {
                break;
            }
        }
        if (ready as i32) <= 0 {
            return ready;
        }
        if live != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(FLUSH, u32, live, rd32(this.wrapping_add(INDEX_OFF)));
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(COMMIT, u32, this);
        ready
    }
});
