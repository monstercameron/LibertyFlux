// original: 0x00877E70 rage::crmtComposerOptimized::vf14

/// Record or dispatch a table-block copy operation (opcode 7).
///
/// `this` points to a composer object. In immediate mode (flag byte at
/// `+0x98` non-zero) the helper runs at once with (`this`, `a0`, `a1`,
/// `a2`, `a3`). Otherwise a node is popped from the free list at `+0x88`
/// (refilled through vtable slot `+0x78` when empty); when the block index
/// at `+0x90` has reached 4 (SIGNED comparison: negative indexes skip the
/// refill) the pool is refilled a second time. The index is then bumped
/// and a copy helper runs with (block, `a1`, `a0 * 4`), where block is
/// `inner + 0xC1C + index * 128` and `inner` is the word at `+0x0c`. The
/// node is stamped with opcode 5, given block at `+8`, `a3` at `+0xc`,
/// `a0` at `+0x10` and `a2` at `+0x14`, and appended to the pending list
/// (`+0x80`/`+0x84`).
///
/// Original: 0x00877E70 (thiscall, four stack arguments). No return value.
lf_checker_rt::export!(thiscall, rw_00877E70(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x80;
        const TAIL_OFF: u32 = 0x84;
        const FREE_OFF: u32 = 0x88;
        const IMMED_OFF: u32 = 0x98;
        const NEXT_OFF: u32 = 4;
        const REFILL_VT_SLOT: u32 = 0x78;
        const INNER_OFF: u32 = 0x0c;
        const BLOCK_INDEX_OFF: u32 = 0x90;
        const BLOCK_BASE: u32 = 0xc1c;
        const BLOCK_SHIFT: u32 = 7;
        const REFILL_AT: i32 = 4;
        const OP_BLK_OFF: u32 = 0x08;
        const OP_A3_OFF: u32 = 0x0c;
        const OP_A0_OFF: u32 = 0x10;
        const OP_A2_OFF: u32 = 0x14;
        const OPCODE: u32 = 7;
        const HELPER: u32 = 2;
        const COPIER: u32 = 3;
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

        if rd8(this.wrapping_add(IMMED_OFF)) != 0 {
            lf_checker_rt::callee_thiscall!(HELPER, u32, this, a0, a1, a2, a3);
        } else {
            if rd32(this.wrapping_add(FREE_OFF)) == 0 {
                vcall0(this, REFILL_VT_SLOT);
            }
            let node = rd32(this.wrapping_add(FREE_OFF));
            let next = rd32(node.wrapping_add(NEXT_OFF));
            wr32(this.wrapping_add(FREE_OFF), next);
            wr32(node.wrapping_add(NEXT_OFF), 0);
            if (rd32(this.wrapping_add(BLOCK_INDEX_OFF)) as i32) >= REFILL_AT {
                vcall0(this, REFILL_VT_SLOT);
            }
            let index = rd32(this.wrapping_add(BLOCK_INDEX_OFF));
            wr32(this.wrapping_add(BLOCK_INDEX_OFF), index.wrapping_add(1));
            let inner = rd32(this.wrapping_add(INNER_OFF));
            let block = inner
                .wrapping_add(BLOCK_BASE)
                .wrapping_add(index.wrapping_shl(BLOCK_SHIFT));
            lf_checker_rt::callee_cdecl!(
                COPIER, u32, block, a1, a0.wrapping_mul(4));
            wr32(node, OPCODE);
            wr32(node.wrapping_add(OP_BLK_OFF), block);
            wr32(node.wrapping_add(OP_A3_OFF), a3);
            wr32(node.wrapping_add(OP_A0_OFF), a0);
            wr32(node.wrapping_add(OP_A2_OFF), a2);
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
