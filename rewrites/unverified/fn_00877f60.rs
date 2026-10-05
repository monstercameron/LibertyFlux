// original: 0x00877F60 rage::crmtComposerOptimized::vf15

/// Record or dispatch a conditional node link (opcode 8).
///
/// `this` points to a composer object. In immediate mode (flag byte at
/// `+0x98` non-zero) the helper runs at once with (`this`, `a0`).
/// Otherwise, when `a0` is non-zero a node is popped from the free list
/// at `+0x88` (refilled through vtable slot `+0x78` when empty), stamped
/// with opcode 8, given `a0` at `+0xc`, and linked into the pending list
/// through the append helper; a null `a0` records nothing.
///
/// Original: 0x00877F60 (thiscall, one stack argument). No return value.
lf_checker_rt::export!(thiscall, rw_00877F60(this: u32, a0: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x80;
        const TAIL_OFF: u32 = 0x84;
        const FREE_OFF: u32 = 0x88;
        const IMMED_OFF: u32 = 0x98;
        const NEXT_OFF: u32 = 4;
        const REFILL_VT_SLOT: u32 = 0x78;
        const OP_A0_OFF: u32 = 0x0c;
        const OPCODE: u32 = 8;
        const HELPER: u32 = 2;
        const APPENDER: u32 = 3;
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
            lf_checker_rt::callee_thiscall!(HELPER, u32, this, a0);
        } else if a0 != 0 {
            if rd32(this.wrapping_add(FREE_OFF)) == 0 {
                vcall0(this, REFILL_VT_SLOT);
            }
            let node = rd32(this.wrapping_add(FREE_OFF));
            let next = rd32(node.wrapping_add(NEXT_OFF));
            wr32(this.wrapping_add(FREE_OFF), next);
            wr32(node.wrapping_add(NEXT_OFF), 0);
            wr32(node, OPCODE);
            wr32(node.wrapping_add(OP_A0_OFF), a0);
            lf_checker_rt::callee_thiscall!(APPENDER, u32, this, node);
        }
        0
    }
});
