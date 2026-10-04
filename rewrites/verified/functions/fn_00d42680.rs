// original: 0x00d42680 helper_slot_store (proposed)

/// Ask the slot helper for a 16-byte slot, then store one value through it.
///
/// Calls the helper with `this+0x04` as its object and a fixed size of 16;
/// the helper returns a slot pointer, and the incoming `value` is stored at
/// the slot address. Returns the slot pointer (left in eax by the original).
///
/// Original: thiscall, one stack word (value), callee pops 4. The helper is
/// thiscall with one stack word and is intercepted by the checker.
lf_checker_rt::export!(thiscall, rw_00d42680(this: u32, value: u32) -> u32 {
    unsafe {
        const SLOT_CALLEE: u32 = 1;
        const SLOT_SIZE: u32 = 0x10;
        const BASE_OFF: u32 = 4;
        let slot: u32 =
            lf_checker_rt::callee_thiscall!(SLOT_CALLEE, u32, this.wrapping_add(BASE_OFF), SLOT_SIZE);
        (slot as *mut u32).write_unaligned(value);
        slot
    }
});
