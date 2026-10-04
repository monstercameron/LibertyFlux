// original: 0x00e622d0 net_gain_fanout
/// Copy one global float into three global slots.
///
/// Loads the single-precision value at `0x01110CD8` and stores it unchanged
/// into `0x01B4A9A0`, `0x01B4A9A4` and `0x01B4A9A8`. Takes no arguments and returns
/// nothing; entry registers are ignored. A plain 32-bit copy: `movss`
/// moves the bits without touching them, so NaNs keep sign and payload.
export!(cdecl, rw_00e622d0() -> u32 {
    unsafe {
        let v = *global::<u32>(0x1110CD8);
        *global::<u32>(0x1B4A9A0) = v;
        *global::<u32>(0x1B4A9A4) = v;
        *global::<u32>(0x1B4A9A8) = v;
        0
    }
});
