// original: 0x008778E0 rage::crmtComposerOptimized::vf7

/// Record or dispatch a 16-byte vector operation (opcode 0xb).
///
/// `this` points to a composer object. In immediate mode (flag byte at
/// `+0x98` non-zero) the operation runs at once: the handler stored in the
/// target object `a0` at `+0x0c` is called with (`a0`, `this`) and the
/// caller cleans the stack (cdecl). Otherwise a node is popped from the
/// free list at `+0x88` (refilled through vtable slot `+0x78` when empty),
/// stamped with opcode 0xb, given `a1` at `+0x10` and the 16 bytes at `a0`
/// at `+0x20`, and appended to the pending list (`+0x80`/`+0x84`).
///
/// Original: 0x008778E0 (thiscall, two stack arguments). No return value.
lf_checker_rt::export!(thiscall, rw_008778E0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x80;
        const TAIL_OFF: u32 = 0x84;
        const FREE_OFF: u32 = 0x88;
        const IMMED_OFF: u32 = 0x98;
        const NEXT_OFF: u32 = 4;
        const REFILL_VT_SLOT: u32 = 0x78;
        const HANDLER_OFF: u32 = 0x0c;
        const OP_ARG_OFF: u32 = 0x10;
        const OP_VEC_OFF: u32 = 0x20;
        const OPCODE: u32 = 0xb;
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
            let target = rd32(a0.wrapping_add(HANDLER_OFF));
            let handler: extern "cdecl" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            handler(a0, this);
        } else {
        if rd32(this.wrapping_add(FREE_OFF)) == 0 {
            vcall0(this, REFILL_VT_SLOT);
        }
        let node = rd32(this.wrapping_add(FREE_OFF));
        let next = rd32(node.wrapping_add(NEXT_OFF));
        wr32(this.wrapping_add(FREE_OFF), next);
        wr32(node.wrapping_add(NEXT_OFF), 0);
        wr32(node, OPCODE);
        wr32(node.wrapping_add(OP_ARG_OFF), a1);
        wr32(node.wrapping_add(OP_VEC_OFF), rd32(a0));
        wr32(node.wrapping_add(OP_VEC_OFF + 4), rd32(a0.wrapping_add(4)));
        wr32(node.wrapping_add(OP_VEC_OFF + 8), rd32(a0.wrapping_add(8)));
        wr32(node.wrapping_add(OP_VEC_OFF + 12), rd32(a0.wrapping_add(12)));
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
