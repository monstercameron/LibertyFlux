// original: 0x00877980 rage::crmtComposerOptimized::vf8

/// Record or dispatch a two-float scalar operation (opcode 1).
///
/// `this` points to a composer object. In immediate mode (flag byte at
/// `+0x98` non-zero) the helper runs at once with (`this`, `a0`, `a3`,
/// 0) on the stack and the two floats `a1`/`a2` in XMM2/XMM3 (the callee
/// reads them there: its first stores spill both). The rewrite smuggles
/// the floats to the stub through stack slots 0-1, so the stack arguments
/// go uncompared and only ECX, XMM2 and XMM3 are observed. Otherwise a
/// node is popped from the free list at `+0x88` (refilled through vtable
/// slot `+0x78` when empty), stamped with opcode 1, given `a0` at `+8`,
/// `a1` at `+0x10`, `a2` at `+0x14` and the low byte of `a3` at `+0x18`
/// (with `+0xc` and `+0x1c` cleared), and appended to the pending list
/// (`+0x80`/`+0x84`). Floats are only moved, never computed on.
///
/// Original: 0x00877980 (thiscall, four stack arguments). No return value.
lf_checker_rt::export!(thiscall, rw_00877980(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x80;
        const TAIL_OFF: u32 = 0x84;
        const FREE_OFF: u32 = 0x88;
        const IMMED_OFF: u32 = 0x98;
        const NEXT_OFF: u32 = 4;
        const REFILL_VT_SLOT: u32 = 0x78;
        const OP_A0_OFF: u32 = 0x08;
        const OP_F1_OFF: u32 = 0x10;
        const OP_F2_OFF: u32 = 0x14;
        const OP_B_OFF: u32 = 0x18;
        const OP_C_OFF: u32 = 0x0c;
        const OP_Z_OFF: u32 = 0x1c;
        const OPCODE: u32 = 1;
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

        if rd8(this.wrapping_add(IMMED_OFF)) != 0 {
            lf_checker_rt::callee_thiscall!(HELPER, u32, this, a1, a2, 0);
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
        wr8(node.wrapping_add(OP_B_OFF), a3 as u8);
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
