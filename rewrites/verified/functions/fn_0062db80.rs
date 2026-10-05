// original: 0x0062DB80 skyhat_set_param_20 (proposed)

/// Store one float into the parameter block reached through this object.
///
/// `this` points to the owner; the float at `PARAM_BLOCK` (`+0x44`) holds the
/// address of a parameter block, and the incoming float (passed by value on
/// the stack, moved as raw bits) is written to `SLOT` (`+32`) of that block.
/// Nothing else is read or written. Returns the parameter block address left
/// in the accumulator by the load (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0062db80(this: u32, value_bits: u32) -> u32 {
    unsafe {
        const PARAM_BLOCK: u32 = 0x44;
        const SLOT: u32 = 0x20;
        let block = ((this + PARAM_BLOCK) as *const u32).read_unaligned();
        ((block + SLOT) as *mut u32).write_unaligned(value_bits);
        block
    }
});
