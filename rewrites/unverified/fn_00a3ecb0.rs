// original: 0x00A3ECB0 vehicle_accept_attachment_state

/// Return whether the receiver's state word at `0x12d0` is one of the three
/// accepted values: zero, one, or seven. A flag at `0x0f18` bit `0x10`
/// rejects the receiver before the state is read. The result is an 8-bit
/// boolean, matching the original `AL` return.
///
/// The original is a thiscall method with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a3ecb0(this: u32) -> u8 {
    const STATE_FLAGS: u32 = 0x0f18;
    const REJECT_MASK: u8 = 0x10;
    const STATE: u32 = 0x12d0;

    unsafe {
        let flags = (this.wrapping_add(STATE_FLAGS) as *const u8).read();
        if flags & REJECT_MASK != 0 {
            return 0;
        }
        let state = (this.wrapping_add(STATE) as *const u32).read_unaligned();
        u8::from(state == 0 || state == 1 || state == 7)
    }
});
