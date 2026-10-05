// original: 0x00877A40 rage::crmtComposerOptimized::vf9

/// Record or dispatch an object-plus-floats operation (opcode 2).
///
/// `this` points to a composer object. In immediate mode (flag byte at
/// `+0x98` non-zero) the sequence counter at `+0x72` grows by 2, the inner
/// helper runs with (`inner + 4`, `inner`) where `inner` is the word at
/// `+0x0c`, and the target object `a0`'s virtual at slot `+0x1c` runs with
/// (`a0`, helper-answer, `a1`, `a2`). Otherwise a node is popped from the
/// free list at `+0x88` (refilled through vtable slot `+0x78` when
/// empty), stamped with opcode 2, given `a0` at `+8`, `a1` at `+0x10` and
/// `a2` at `+0x14` (with `+0xc` and `+0x1c` cleared), and appended to the
/// pending list (`+0x80`/`+0x84`). Floats are only moved, never computed.
///
/// Original: 0x00877A40 (thiscall, three stack arguments). No return value.
lf_checker_rt::export!(thiscall, rw_00877A40(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x80;
        const TAIL_OFF: u32 = 0x84;
        const FREE_OFF: u32 = 0x88;
        const IMMED_OFF: u32 = 0x98;
        const NEXT_OFF: u32 = 4;
        const REFILL_VT_SLOT: u32 = 0x78;
        const INNER_OFF: u32 = 0x0c;
        const SEQ_OFF: u32 = 0x72;
        const SEQ_STEP: u16 = 2;
        const TARGET_VT_SLOT: u32 = 0x1c;
        const OP_A0_OFF: u32 = 0x08;
        const OP_F1_OFF: u32 = 0x10;
        const OP_F2_OFF: u32 = 0x14;
        const OP_C_OFF: u32 = 0x0c;
        const OP_Z_OFF: u32 = 0x1c;
        const OPCODE: u32 = 2;
        const INNER_HELPER: u32 = 2;
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
            wr16(this.wrapping_add(SEQ_OFF), seq.wrapping_add(SEQ_STEP));
            let inner = rd32(this.wrapping_add(INNER_OFF));
            let answer = lf_checker_rt::callee_thiscall!(
                INNER_HELPER, u32, inner.wrapping_add(4), inner);
            let vtable = rd32(a0);
            let target: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(
                    rd32(vtable.wrapping_add(TARGET_VT_SLOT)) as usize,
                ) };
            target(a0, answer, a1, a2);
        } else {
        if rd32(this.wrapping_add(FREE_OFF)) == 0 {
            vcall0(this, REFILL_VT_SLOT);
        }
        let node = rd32(this.wrapping_add(FREE_OFF));
        let next = rd32(node.wrapping_add(NEXT_OFF));
        wr32(this.wrapping_add(FREE_OFF), next);
        wr32(node.wrapping_add(NEXT_OFF), 0);
        wr32(node, OPCODE);
        wr32(node.wrapping_add(OP_A0_OFF), a0);
        wr32(node.wrapping_add(OP_F1_OFF), a1);
        wr32(node.wrapping_add(OP_F2_OFF), a2);
        wr32(node.wrapping_add(OP_C_OFF), 0);
        wr8(node.wrapping_add(OP_Z_OFF), 0);
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
