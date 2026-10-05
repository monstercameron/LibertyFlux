// original: 0x00877BE0 rage::crmtComposerOptimized::vf11

/// Record or dispatch a clamped-float operation (opcode 4).
///
/// `this` points to a composer object. In immediate mode (flag byte at
/// `+0x98` non-zero) the sequence counter at `+0x72` grows by 1, the index
/// at `inner + 0x14` (where `inner` is the word at `+0x0c`) is decremented,
/// and the helper runs with the two-word frame (table[index - 1], `a1`)
/// as `this`, `a1` and table[index] on the stack, and `a0` clamped to
/// [0, K] in XMM3, where K is the shipped float constant. The clamp keeps
/// NaN as NaN: the first compare branches on ordered-above (`ja`), the
/// second on below-or-equal-or-unordered (`jbe`), so the rewrite uses
/// `!(0.0 > a0)` then `!(a0 > K)` to match unordered inputs exactly. The
/// floats reach the stub through stack slot 0, so the stack arguments go
/// uncompared and only the snapshot and XMM3 are observed. The original
/// builds its frame over its incoming `a0` slot, so the stack check is
/// off for this function (the stored value is observed in the snapshot).
/// Otherwise a node is popped from the free list at `+0x88` (refilled
/// through vtable slot `+0x78` when empty), stamped with opcode 4, given
/// `a0` at `+8` and `a1` at `+0xc`, and appended to the pending list
/// (`+0x80`/`+0x84`).
///
/// Original: 0x00877BE0 (thiscall, two stack arguments). No return value.
lf_checker_rt::export!(thiscall, rw_00877BE0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x80;
        const TAIL_OFF: u32 = 0x84;
        const FREE_OFF: u32 = 0x88;
        const IMMED_OFF: u32 = 0x98;
        const NEXT_OFF: u32 = 4;
        const REFILL_VT_SLOT: u32 = 0x78;
        const INNER_OFF: u32 = 0x0c;
        const SEQ_OFF: u32 = 0x72;
        const TABLE_OFF: u32 = 0x04;
        const INDEX_OFF: u32 = 0x14;
        const CLAMP_MAX_VA: u32 = 0x00fe88e8;
        const OP_F_OFF: u32 = 0x08;
        const OP_A1_OFF: u32 = 0x0c;
        const OPCODE: u32 = 4;
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
            let index = rd32(inner.wrapping_add(INDEX_OFF));
            wr32(inner.wrapping_add(INDEX_OFF), index.wrapping_sub(1));
            let table = rd32(inner.wrapping_add(TABLE_OFF));
            let prev = rd32(table.wrapping_add(index.wrapping_sub(1).wrapping_mul(4)));
            let cur = rd32(table.wrapping_add(index.wrapping_mul(4)));
            let clamp_max =
                f32::from_bits(rd32(lf_checker_rt::relocated(CLAMP_MAX_VA)));
            let a0f = f32::from_bits(a0);
            let clamped = if !(0.0f32 > a0f) {
                if !(a0f > clamp_max) {
                    a0
                } else {
                    clamp_max.to_bits()
                }
            } else {
                0u32
            };
            let pad = [prev, a1];
            lf_checker_rt::callee_thiscall!(
                HELPER, u32, pad.as_ptr() as u32, clamped, cur);
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
