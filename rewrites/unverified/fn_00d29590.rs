// original: 0x00d29590 targeting_slot_reset (proposed)

/// Reset one slot through the vtable hook, then clear its entry.
///
/// Calls the slot hook in vtable slot 6 (planted by the contract) with
/// `index`, then clears the slot entry at `this + 0x20 + index * 64` with
/// the entry-clear helper (intercepted). The original leaves `eax` untouched,
/// so no return channel is compared.
///
/// Original: 0x00D29590 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00d29590(this: u32, index: u32) -> u32 {
    unsafe {
        const HOOK_SLOT: u32 = 0x18;
        const ENTRY_BASE: u32 = 0x20;
        const ENTRY_SHIFT: u32 = 6;
        let vt = unsafe { (this as *const u32).read_unaligned() };
        let hook: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(((vt + HOOK_SLOT) as *const u32).read_unaligned() as usize) };
        hook(this, index);
        let entry = this + ENTRY_BASE + (index << ENTRY_SHIFT);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, entry);
        0
    }
});
