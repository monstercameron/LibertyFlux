// original: 0x00878100 rage::crmtComposerOptimized::vf18

/// Record or dispatch a marker operation (opcode 0xc).
///
/// `this` points to a composer object. In immediate mode (flag byte at
/// `+0x98` non-zero) the sequence counter at `+0x72` grows by 1 and the
/// inner helper runs with (`inner + 4`, `this`) where `inner` is the word
/// at `+0x0c`. Otherwise a node is popped from the free list at `+0x88`
/// (refilled through vtable slot `+0x78` when empty), stamped with opcode
/// 0xc, and appended to the pending list (`+0x80`/`+0x84`) with no
/// arguments.
///
/// Original: 0x00878100 (thiscall, no stack arguments). No return value.
lf_checker_rt::export!(thiscall, rw_00878100(this: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x80;
        const TAIL_OFF: u32 = 0x84;
        const FREE_OFF: u32 = 0x88;
        const IMMED_OFF: u32 = 0x98;
        const NEXT_OFF: u32 = 4;
        const REFILL_VT_SLOT: u32 = 0x78;
        const INNER_OFF: u32 = 0x0c;
        const SEQ_OFF: u32 = 0x72;
        const OPCODE: u32 = 0xc;
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
            wr16(this.wrapping_add(SEQ_OFF), seq.wrapping_add(1));
            let inner = rd32(this.wrapping_add(INNER_OFF));
            lf_checker_rt::callee_thiscall!(
                INNER_HELPER, u32, inner.wrapping_add(4), this);
        } else {
        if rd32(this.wrapping_add(FREE_OFF)) == 0 {
            vcall0(this, REFILL_VT_SLOT);
        }
        let node = rd32(this.wrapping_add(FREE_OFF));
        let next = rd32(node.wrapping_add(NEXT_OFF));
        wr32(this.wrapping_add(FREE_OFF), next);
        wr32(node.wrapping_add(NEXT_OFF), 0);
        wr32(node, OPCODE);
        let _ = node;
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
