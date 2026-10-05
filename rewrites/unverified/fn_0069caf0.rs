// original: 0x0069CAF0 rage::crAnimChannelCurveFloat::purge_list

/// Drains a curve-float node list, releasing every node.
///
/// `this` is the list `{head+0, tail+4, count+8}`. While the head is
/// non-null the first node is unlinked (its link cleared, the tail
/// cleared when it was the last node, the count decremented) and
/// released through the thread allocator reached as
/// `tls[0] -> [+8] -> vtable[+0xC]` (callee 1, thiscall: allocator,
/// node). An empty list returns immediately with whatever `eax` held on
/// entry (unverifiable, so the contract only exercises non-empty lists).
/// Returns the last release answer.
///
/// Original: 0x0069CAF0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0069CAF0(this: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const ALLOC_OBJ_OFF: u32 = 8;
        const FREE_SLOT: u32 = 0x0C;
        const TAIL_OFF: u32 = 4;
        const LINK_OFF: u32 = 4;
        const COUNT_OFF: u32 = 8;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let mut head = rd32(this);
        if head == 0 {
            return 0; // placeholder: original returns entry-eax residue (unverifiable, uncovered)
        }
        let tls_base = lf_checker_rt::tls_slot(TLS_SLOT);
        let heap_obj = rd32(tls_base + ALLOC_OBJ_OFF);
        let vtable = rd32(heap_obj);
        let free: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vtable + FREE_SLOT) as usize) };
        let mut last: u32 = 0;
        loop {
            let node = head;
            head = rd32(node + LINK_OFF);
            wr32(this, head);
            wr32(node + LINK_OFF, 0);
            if rd32(this + TAIL_OFF) == node {
                wr32(this + TAIL_OFF, 0);
            }
            wr32(this + COUNT_OFF, rd32(this + COUNT_OFF).wrapping_sub(1));
            last = free(heap_obj, node);
            if head == 0 {
                break;
            }
        }
        last
    }
});
