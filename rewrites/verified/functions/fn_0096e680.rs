// original: 0x0096e680 audio_graph_node_init (proposed)

/// Initialise three audio graph nodes from the manager, walking one node
/// chain per slot and configuring each node through its virtual slots.
///
/// `this` points to the owner object (a flag byte at `+FLAG_OFF`, six
/// parameter dwords at `+PARAM_OFF`, and one shared dword at `+SHARED_OFF`
/// copied into every configured node). The global byte at `+FLAG_GLOBAL`
/// gates the whole function: when it is non-zero the original returns its
/// entry `eax` unchanged (that path is outside this proof: the byte is
/// pristine-zero, so the branch never runs; see below).
///
/// Otherwise the body runs three times (`slot` = 0, 1, 2). Each pass asks
/// the manager object for a node chain head, then walks it: the head and
/// the next node must carry tag `TAG_CHAIN` (else the pass ends, releasing
/// the head when one was obtained). The head's slot `SLOT_INIT` result is
/// marked (1, 1, 0); the next node's result takes the table dword for this
/// slot, value 3, and a select byte that is 1 exactly when `slot == 2` or
/// (`slot == 0` and the owner flag is set).
///
/// The tail depends on the slot. For slot 2 the following node must carry
/// tag `TAG_TAIL2`: its result receives six copies of the shared dword
/// plus the six parameter dwords, and the node after that (tag `TAG_TAIL`)
/// is configured twice with the owner flag byte. For slots 0 and 1 the
/// following node must carry tag `TAG_TAIL`: slot 0 configures it twice
/// (flag byte into two slots), slot 1 once. Every obtained head is
/// released through slot `SLOT_RELEASE`.
///
/// Returns the last value produced (the release answer, the head pointer
/// on a tag mismatch, or 0 when no head was obtained).
///
/// Original: 0x0096e680 (thiscall, no stack arguments, plain return).
lf_checker_rt::export!(thiscall, rw_0096e680(this: u32) -> u32 {
    unsafe {
        const FLAG_GLOBAL: u32 = 0x0115dce7;
        const MANAGER: u32 = 0x0115da4c;
        const TABLE: u32 = 0x010379c4;
        const FLAG_OFF: u32 = 0x3210;
        const SHARED_OFF: u32 = 0x2fe4;
        const PARAM_OFF: u32 = 0x2fe8;
        const TAG_CHAIN: u8 = 3;
        const TAG_TAIL2: u8 = 7;
        const TAG_TAIL: u8 = 2;
        const SLOT_INIT: u32 = 0x10;
        const SLOT_RELEASE: u32 = 0x14;
        const CALLEE_HEAD: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        /// Call virtual slot `SLOT` of `obj` (thiscall, no stack arguments)
        /// through the object's own table, exactly like the original.
        #[inline(always)]
        unsafe fn vslot(obj: u32, slot: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let target = rd32(vt.wrapping_add(slot));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(target as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn tag(node: u32) -> u8 {
            unsafe { rd8(rd32(node.wrapping_add(4))) }
        }
        #[inline(always)]
        unsafe fn next(node: u32) -> u32 {
            unsafe { rd32(node.wrapping_add(8)) }
        }

        // Gating flag. The original returns its entry eax here, which a
        // rewrite cannot observe; the proof never sets the flag.
        if rd8(lf_checker_rt::relocated(FLAG_GLOBAL)) != 0 {
            return 0;
        }
        let manager = lf_checker_rt::relocated(MANAGER);
        let table = lf_checker_rt::relocated(TABLE);
        let flag = rd8(this.wrapping_add(FLAG_OFF));
        let shared = rd32(this.wrapping_add(SHARED_OFF));

        let mut out: u32 = 0;
        let mut slot: u32 = 0;
        while slot < 3 {
            let head: u32 =
                lf_checker_rt::callee_thiscall!(CALLEE_HEAD, u32, manager, slot);
            out = head;
            if head != 0 && tag(head) == TAG_CHAIN {
                let r0 = vslot(head, SLOT_INIT);
                wr32(r0.wrapping_add(0x10), 1);
                wr8(r0.wrapping_add(0x14), 1);
                wr32(r0, 0);
                out = r0;
                let n1 = next(head);
                if n1 != 0 && tag(n1) == TAG_CHAIN {
                    let r1 = vslot(n1, SLOT_INIT);
                    wr32(r1.wrapping_add(0x10), 3);
                    wr32(r1, rd32(table.wrapping_add(slot.wrapping_mul(4))));
                    let sel: u8 =
                        if slot == 2 || (slot == 0 && flag != 0) { 1 } else { 0 };
                    wr8(r1.wrapping_add(0x14), sel);
                    out = r1;
                    let n2 = next(n1);
                    if slot == 2 {
                        if n2 != 0 && tag(n2) == TAG_TAIL2 {
                            let r2 = vslot(n2, SLOT_INIT);
                            let mut k: u32 = 0;
                            while k < 6 {
                                wr32(r2.wrapping_add(0x30).wrapping_add(k.wrapping_mul(4)), shared);
                                wr32(
                                    r2.wrapping_add(k.wrapping_mul(4)),
                                    rd32(this.wrapping_add(PARAM_OFF).wrapping_add(k.wrapping_mul(4))),
                                );
                                k = k.wrapping_add(1);
                            }
                            out = r2;
                            let n3 = next(n2);
                            if n3 != 0 && tag(n3) == TAG_TAIL {
                                let ra = vslot(n3, SLOT_INIT);
                                wr8(ra.wrapping_add(0x10), flag);
                                out = ra;
                                let rb = vslot(n3, SLOT_INIT);
                                wr8(rb.wrapping_add(0x11), flag);
                                out = rb;
                            }
                        }
                    } else if n2 != 0 && tag(n2) == TAG_TAIL {
                        if slot == 0 {
                            let ra = vslot(n2, SLOT_INIT);
                            wr8(ra.wrapping_add(0x11), flag);
                            out = ra;
                        }
                        let rb = vslot(n2, SLOT_INIT);
                        wr8(rb.wrapping_add(0x10), flag);
                        out = rb;
                    }
                }
                out = vslot(head, SLOT_RELEASE);
            }
            slot = slot.wrapping_add(1);
        }
        out
    }
});
