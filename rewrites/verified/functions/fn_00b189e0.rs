// original: 0x00b189e0 lazy_link_or_forward (proposed)

/// Links a value into the object's slot once, then forwards the slot.
///
/// Thiscall of one stack word (callee pops 4 bytes). When the dword at
/// +0x284 is already nonzero it is left alone and its address is
/// returned. Otherwise the argument is stored there and control
/// tail-calls the consumer with the slot address as its single stack
/// argument, returning the consumer's result.
lf_checker_rt::export!(thiscall, rw_00b189e0(this: u32, value: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x284;
        let field = this.wrapping_add(SLOT);
        if (field as *const u32).read_unaligned() != 0 {
            field
        } else {
            (field as *mut u32).write_unaligned(value);
            lf_checker_rt::callee_stdcall!(1, u32, field)
        }
    }
});
