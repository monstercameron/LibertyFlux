// original: 0x00a96560 fade_ready_test
/// Tests whether the channel is ready: idle state and threshold reached.
///
/// A non-zero state word is never ready. Otherwise readiness is the
/// threshold byte at offset 0x1B reaching 200. Only the low byte of the
/// result is defined.
export!(thiscall, rw_00a96560(this: u32) -> u32 {
    unsafe {
        if *(this as *const u32) != 0 {
            0
        } else if *((this + 0x1B) as *const u8) >= 0xC8 {
            1
        } else {
            0
        }
    }
});
