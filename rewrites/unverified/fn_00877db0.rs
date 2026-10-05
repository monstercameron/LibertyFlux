// original: 0x00877DB0 rage::crmtComposerOptimized::vf13

/// Record or dispatch a float-plus-word operation (opcode 6).
///
/// `this` points to a composer object. In immediate mode (flag byte at
/// `+0x98` non-zero) the sequence counter at `+0x72` grows by 1, the index
/// at `this + 0x14` is decremented, and the helper runs with the one-word
/// frame (table[inner-index]) as `this`, `a0` in XMM3 and table2[index]
/// on the stack, where table is `inner + 4` (`inner` is the word at
/// `+0x0c`) and table2 is the word at `this + 4`. The float reaches the
/// stub through stack slot 0, so the stack arguments go uncompared and
/// only the snapshot and XMM3 are observed. Otherwise a node is popped
/// from the free list at `+0x88` (refilled through vtable slot `+0x78`
/// when empty), stamped with opcode 6, given `a0` at `+8` and `a1` at
/// `+0xc`, and appended to the pending list (`+0x80`/`+0x84`). The float
/// is only moved, never computed on.
///
/// Original: 0x00877DB0 (thiscall, two stack arguments). No return value.
lf_checker_rt::export!(thiscall, rw_00877DB0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x80;
        const TAIL_OFF: u32 = 0x84;
        const FREE_OFF: u32 = 0x88;
        const IMMED_OFF: u32 = 0x98;
        const NEXT_OFF: u32 = 4;
        const REFILL_VT_SLOT: u32 = 0x78;
        const INNER_OFF: u32 = 0x0c;
        const T2_OFF: u32 = 0x04;
        const SEQ_OFF: u32 = 0x72;
        const TABLE_OFF: u32 = 0x04;
        const INDEX_OFF: u32 = 0x14;
        const OP_F_OFF: u32 = 0x08;
        const OP_A1_OFF: u32 = 0x0c;
        const OPCODE: u32 = 6;
        const HELPER: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn vcall0(this: u32, slot: u32) -> u32 {
            unsafe {
                let vtable = (this as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    unsafe { core::mem::transmute(
                        ((vtable.wrapping_add(slot)) as *const u32).read_unaligned() as usize,
                    ) };
                f(this)
            }
        }

        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        if rd8(this.wrapping_add(IMMED_OFF)) != 0 {
            let seq = rd16(this.wrapping_add(SEQ_OFF));
            wr16(this.wrapping_add(SEQ_OFF), seq.wrapping_add(1));
            let inner = rd32(this.wrapping_add(INNER_OFF));
            let index = rd32(this.wrapping_add(INDEX_OFF));
            wr32(this.wrapping_add(INDEX_OFF), index.wrapping_sub(1));
            let table = rd32(inner.wrapping_add(TABLE_OFF));
            let inner_index = rd32(inner.wrapping_add(INDEX_OFF));
            let framed = rd32(table.wrapping_add(inner_index.wrapping_mul(4)));
            let table2 = rd32(this.wrapping_add(T2_OFF));
            let item = rd32(table2.wrapping_add(index.wrapping_mul(4)));
            // The original's second frame slot is its own return address;
            // only the first word is observed (snapshot).
            let pad = [framed, 0u32];
            lf_checker_rt::callee_thiscall!(
                HELPER, u32, pad.as_ptr() as u32, a0, item);
        } else {
        if rd32(this.wrapping_add(FREE_OFF)) == 0 {
            vcall0(this, REFILL_VT_SLOT);
        }
        let node = rd32(this.wrapping_add(FREE_OFF));
        let next = rd32(node.wrapping_add(NEXT_OFF));
        wr32(this.wrapping_add(FREE_OFF), next);
        wr32(node.wrapping_add(NEXT_OFF), 0);
        wr32(node, OPCODE);
        wr32(node.wrapping_add(OP_F_OFF), a0);
        wr32(node.wrapping_add(OP_A1_OFF), a1);
        let tail = rd32(this.wrapping_add(TAIL_OFF));
        let mut slot = if tail != 0 {
            tail.wrapping_add(NEXT_OFF)
        } else {
            this.wrapping_add(HEAD_OFF)
        };
        while rd32(slot) != 0 {
            slot = rd32(slot).wrapping_add(NEXT_OFF);
        }
        wr32(slot, node);
        wr32(this.wrapping_add(TAIL_OFF), node);
        }
        0
    }
});
