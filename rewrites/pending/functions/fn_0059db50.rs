// original: 0x0059db50 poll_bit1_when_idle
// True when the state word is 0 and bit 1 of the flags word is set.
//
// The original shifts the whole word right by one and tests the low bit,
// which is exactly a bit-1 test.
export!(cdecl, rw_0059DB50() -> u32 {
    let st = unsafe { *global::<u32>(0x11D6FD4) };
    if st != 0 {
        return 0;
    }
    let flags = unsafe { *global::<u32>(0x18B6EA4) };
    u32::from((flags >> 1) & 1 != 0)
});
