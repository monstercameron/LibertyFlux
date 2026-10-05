// original: 0x008782E0 rage::crmtComposerOptimized::vf26

/// Sync a composer, then forward one word, a zero and `this`
///
/// `this` points to a composer object. Unless immediate mode is on (flag
/// byte at `+0x98`), the pool-sync virtual (slot `+0x78`) runs first;
/// then the helper runs with (`a0`, `0`, `this`) on the stack and `this` in ECX.
/// (The pushed ECX is `this`: the sync helper preserves it.)
///
/// Original: 0x008782E0 (thiscall, 2 stack arguments). No return value.
lf_checker_rt::export!(thiscall, rw_008782E0(this: u32, a0: u32, _a1: u32) -> u32 {
    unsafe {
        const IMMED_OFF: u32 = 0x98;
        const SYNC_VT_SLOT: u32 = 0x78;
        const HELPER: u32 = 2;
        if (this.wrapping_add(IMMED_OFF) as *const u8).read() == 0 {
            let vtable = (this as *const u32).read_unaligned();
            let sync: extern "thiscall" fn(u32) -> u32 = unsafe {
                core::mem::transmute(
                    ((vtable.wrapping_add(SYNC_VT_SLOT)) as *const u32).read_unaligned()
                        as usize,
                )
            };
            sync(this);
        }
        lf_checker_rt::callee_thiscall!(HELPER, u32, this, a0, 0, this);
        0
    }
});
