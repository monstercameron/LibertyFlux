// original: 0x0059db30 poll_bit0_when_idle
// True when the state word is 0 and bit 0 of the flags byte is set.
export!(cdecl, rw_0059DB30() -> u32 {
    let st = unsafe { *global::<u32>(0x11D6FD4) };
    if st != 0 {
        return 0;
    }
    let flags = unsafe { *global::<u8>(0x18B6EA4) };
    u32::from(flags & 1 != 0)
});
