// original: 0x00878200 rage::crmtComposerOptimized::vf22

/// Sync a composer, then dispatch a table entry with fixed arguments.
///
/// `this` points to a composer object. Unless immediate mode is on (flag
/// byte at `+0x98`), the pool-sync virtual (slot `+0x78`) runs first.
/// Then the sequence counter at `+0x72` grows by 1 and the helper runs
/// with the table entry table[index] as `this` (table is `inner + 4` and
/// index is `inner + 0x14`, where `inner` is the word at `+0x0c`) and the
/// words (5, 6, word70, `a0`, 1) on the stack, where word70 is the
/// zero-extended word at `this + 0x70`. The second stack argument is
/// never read.
///
/// Original: 0x00878200 (thiscall, two stack arguments). No return value.
lf_checker_rt::export!(thiscall, rw_00878200(this: u32, a0: u32, _a1: u32) -> u32 {
    unsafe {
        const INNER_OFF: u32 = 0x0c;
        const SEQ_OFF: u32 = 0x72;
        const WORD70_OFF: u32 = 0x70;
        const TABLE_OFF: u32 = 0x04;
        const INDEX_OFF: u32 = 0x14;
        const IMMED_OFF: u32 = 0x98;
        const SYNC_VT_SLOT: u32 = 0x78;
        const HELPER: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        if (this.wrapping_add(IMMED_OFF) as *const u8).read() == 0 {
            let vtable = rd32(this);
            let sync: extern "thiscall" fn(u32) -> u32 = unsafe {
                core::mem::transmute(rd32(vtable.wrapping_add(SYNC_VT_SLOT)) as usize)
            };
            sync(this);
        }
        let seq = (this.wrapping_add(SEQ_OFF) as *const u16).read_unaligned();
        (this.wrapping_add(SEQ_OFF) as *mut u16).write_unaligned(seq.wrapping_add(1));
        let inner = rd32(this.wrapping_add(INNER_OFF));
        let index = rd32(inner.wrapping_add(INDEX_OFF));
        let table = rd32(inner.wrapping_add(TABLE_OFF));
        let entry = rd32(table.wrapping_add(index.wrapping_mul(4)));
        let word70 =
            (this.wrapping_add(WORD70_OFF) as *const u16).read_unaligned() as u32;
        lf_checker_rt::callee_thiscall!(HELPER, u32, entry, 5, 6, word70, a0, 1);
        0
    }
});
