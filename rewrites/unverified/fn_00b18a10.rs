// original: 0x00b18a10 store_link_and_forward (proposed)

/// Stores a value into the object's slot and forwards the slot.
///
/// Thiscall of one stack word (callee pops 4 bytes). Unconditionally
/// writes the argument to the dword at +0x27C, then tail-calls the
/// consumer with the slot address as its single stack argument and
/// returns the consumer's result.
lf_checker_rt::export!(thiscall, rw_00b18a10(this: u32, value: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x27c;
        let field = this.wrapping_add(SLOT);
        (field as *mut u32).write_unaligned(value);
        lf_checker_rt::callee_stdcall!(1, u32, field)
    }
});
