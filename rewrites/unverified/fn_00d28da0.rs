// original: 0x00d28da0 target_slot_sign_flag (proposed)

/// Report the sign flag of a wanted pointer's slot, activating it first.
///
/// Finds the 0-based slot index of `wanted` with the index search
/// (intercepted); a miss returns 0. On a hit, when the low three bits of
/// the slot word at `this + 0x48 + index * 64` are all clear, activates the
/// slot through vtable slot 3 (planted by the contract) with `(index, 1)`.
/// Returns the cell word's low three bits, sign-extended (the original's
/// shift-left-29 then arithmetic-shift-right-29 extracts bits 0-2 and
/// replicates bit 2).
///
/// Original: 0x00D28DA0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00d28da0(this: u32, wanted: u32) -> u32 {
    unsafe {
        const MISS: u32 = 0xffff_ffff;
        const WORD_BASE: u32 = 0x48;
        const WORD_SHIFT: u32 = 6;
        const QUIET_MASK: u8 = 7;
        const HOOK_SLOT: u32 = 0x0c;
        const HOOK_ARG: u32 = 1;
        const FIELD_SHIFT: u32 = 29;
        let idx: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, wanted);
        if idx == MISS {
            return 0;
        }
        let cell = this + WORD_BASE + (idx << WORD_SHIFT);
        if unsafe { (cell as *const u8).read() } & QUIET_MASK == 0 {
            let vt = unsafe { (this as *const u32).read_unaligned() };
            let hook: extern "thiscall" fn(u32, u32, u32) -> u32 = unsafe {
                core::mem::transmute(((vt + HOOK_SLOT) as *const u32).read_unaligned() as usize)
            };
            hook(this, idx, HOOK_ARG);
        }
        let m = unsafe { (cell as *const u32).read_unaligned() };
        ((m.wrapping_shl(FIELD_SHIFT) as i32) >> FIELD_SHIFT) as u32
    }
});
