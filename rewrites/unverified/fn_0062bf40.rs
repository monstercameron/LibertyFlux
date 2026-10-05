// original: 0x0062BF40 streaming_list_rebuild (proposed)

/// Walk the node list at `+0x10` of `this`, attaching a record to each match.
///
/// The list links through `+0` of each node and ends at the head address
/// (`this + 0x10`). For each node the predicate runs (thiscall on the object
/// at `+0x10` of the node through its slot `+0x0c`, with the argument); when
/// its low byte is set, the tag callee runs (slot `+0x14`), a 0x14-byte block
/// is allocated through the thread-local allocator's slot `+8`, the payload
/// (`0xffffffff`, the tag answer, the argument) is stored at block `+8` and
/// the block is linked after the node object's link cell: the object
/// pointer is advanced by 8 and the `+4` links from there (i.e. `+12` of the
/// object) are spliced. A failed
/// allocation faults on both sides alike. Returns the last callee answer
/// (thiscall, one argument).
lf_checker_rt::export!(thiscall, rw_0062bf40(this: u32, arg: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x10;
        const NODE_OBJ: u32 = 0x10;
        const NEXT_LINK: u32 = 0x0c;
        const ALLOC_SIZE: u32 = 0x14;
        const PRED_SLOT: u32 = 0x0c;
        const TAG_SLOT: u32 = 0x14;
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let head = this.wrapping_add(HEAD);
        let mut node = rd(head);
        let mut eax: u32 = 0;
        while node != head {
            let obj = rd(node.wrapping_add(NODE_OBJ));
            let ovt = rd(obj);
            let pa = rd(ovt.wrapping_add(PRED_SLOT));
            let pred: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(pa as usize);
            let r = pred(obj, arg);
            eax = r;
            if r & 0xFF != 0 {
                let ta = rd(ovt.wrapping_add(TAG_SLOT));
                let tag: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(ta as usize);
                let b = tag(obj);
                eax = b;
                let holder = lf_checker_rt::tls_slot(0);
                let frobj = rd(holder.wrapping_add(8));
                let fvt = rd(frobj);
                let fa = rd(fvt.wrapping_add(8));
                let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(fa as usize);
                let mem = alloc(frobj, ALLOC_SIZE, 0x10, 0);
                eax = mem;
                let ecx = mem.wrapping_add(8);
                if ecx != 0 {
                    wr(ecx, 0xFFFFFFFF);
                    wr(ecx.wrapping_add(4), b);
                    wr(ecx.wrapping_add(8), arg);
                }
                let edi = obj.wrapping_add(8);
                let nxt = rd(edi.wrapping_add(4));
                wr(mem, edi);
                wr(mem.wrapping_add(4), nxt);
                wr(nxt, mem);
                wr(edi.wrapping_add(4), mem);
            }
            node = rd(node);
        }
        eax
    }
});
