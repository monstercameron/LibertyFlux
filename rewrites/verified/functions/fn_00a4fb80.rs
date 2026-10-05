// original: 0x00a4fb80 vehicle_slot_pos_call (proposed)

/// Call the slot-position callee for slot `index`, returning the argument.
///
/// Builds the slot address (`this + 0x100 + index * 0xe0`), passes it in
/// ecx with the first stack word to the position callee, and returns that
/// word unchanged (the callee's answer is ignored). Thiscall, two stack
/// words, one callee, argument in eax.
lf_checker_rt::export!(thiscall, rw_00a4fb80(this: u32, a0: u32, index: u32) -> u32 {
    unsafe {
        const SLOTS_BASE: u32 = 0x100;
        const SLOT_STRIDE: u32 = 0xe0;
        const POS: u32 = 1;
        let slot = this
            .wrapping_add(SLOTS_BASE)
            .wrapping_add(index.wrapping_mul(SLOT_STRIDE));
        lf_checker_rt::callee_thiscall!(POS, u32, slot, a0);
        a0
    }
});
