// original: 0x0091C9A0 classify_setting_value
/// Map a small setting value to a mode flag.
///
/// Values 4 and 5 yield 1, 6 yields 2, 7 yields 0, and 8 yields whether the
/// global mode byte at `0x0116C250` differs from `0x72`; every other input
/// yields 0. The original implements this as a jump switch over `value - 4`;
/// the rewrite states the resolved mapping directly.
export!(cdecl, rw_0091c9a0(v: u32) -> u32 {
    unsafe {
        match v {
            4 | 5 => 1,
            6 => 2,
            7 => 0,
            8 => u32::from(*global::<u8>(0x116C250) != 0x72),
            _ => 0,
        }
    }
});
