// original: 0x00B13B10 lookup_child_state_byte

/// Resolve a child index and return the byte at that position in the state
/// array beginning at `this+0x218`. An index of `u32::MAX` yields zero;
/// otherwise the index is added as an unsigned 32-bit offset and the byte is
/// zero-extended. The helper is a thiscall taking the original query argument.
lf_checker_rt::export!(thiscall, rw_00B13B10(this: u32, query: u32) -> u32 {
    unsafe {
        const STATE_ARRAY: u32 = 0x218;
        let index = lf_checker_rt::callee_thiscall!(1, u32, this, query);
        if index == u32::MAX {
            return 0;
        }
        let byte_address = this.wrapping_add(index).wrapping_add(STATE_ARRAY);
        (byte_address as *const u8).read() as u32
    }
});
