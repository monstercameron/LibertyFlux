// original: 0x00607910 streaming_get_optional_state_byte

/// Returns the state byte at offset 0x111 of the optional object referenced
/// by the primary object's field at 0x78. When present, the original keeps
/// the pointer's high 24 bits in EAX and replaces only AL with that byte.
lf_checker_rt::export!(thiscall, rw_00607910(this: u32) -> u32 {
    const CHILD: u32 = 0x78;
    const STATE_BYTE: u32 = 0x111;
    unsafe {
        let child = (this.wrapping_add(CHILD) as *const u32).read_unaligned();
        if child == 0 {
            0
        } else {
            let state = (child.wrapping_add(STATE_BYTE) as *const u8).read();
            (child & 0xFFFF_FF00) | u32::from(state)
        }
    }
});
