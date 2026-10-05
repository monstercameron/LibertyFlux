// original: 0x009e2c90 ped_slot_store_tail (proposed)

/// Store one word into the first free slot of a four-slot table at
/// `this + 0xd58`, then tail-call the list-link routine with the stored
/// value as its object and the slot address as its argument.
///
/// Scans the four dwords for the first zero; if all four are nonzero it
/// returns 4 immediately (`thiscall`, one stack word kept: the callee pops 4 bytes).
/// Otherwise the argument word is written into the free slot, the stack
/// argument is replaced by the slot's address, `ecx` is set to the stored
/// value, and control jumps (E9 tail call) to the callee, whose return
/// value becomes this function's result.
lf_checker_rt::export!(thiscall, rw_009e2c90(this: u32, value: u32) -> u32 {
    unsafe {
        const SLOT_BASE: u32 = 0xd58;
        const SLOT_COUNT: u32 = 4;
        const TAIL_CALLEE: u32 = 1;
        let mut index: u32 = 0;
        let mut slot: u32 = this.wrapping_add(SLOT_BASE);
        loop {
            if (slot as *const u32).read_unaligned() == 0 {
                break;
            }
            index += 1;
            slot = slot.wrapping_add(4);
            if index >= SLOT_COUNT {
                return SLOT_COUNT;
            }
        }
        (slot as *mut u32).write_unaligned(value);
        lf_checker_rt::callee_thiscall!(TAIL_CALLEE, u32, value, slot)
    }
});
