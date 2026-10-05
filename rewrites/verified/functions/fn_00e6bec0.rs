// original: 0x00e6bec0 veh_fanout_copy_2
/// Copy two global floats out to 3 global slots (fan-out).
///
/// Loads `SRC1` (0x01050AB4) and stores its bits into 0x0171B9B0, 0x0171B9B4, then loads
/// `SRC2` (0x01050AB8) and stores it into 0x0171B9B8. The original moves the
/// values through XMM0 with `movss`; a plain 32-bit copy is the same
/// bits. Takes no arguments, leaves EAX untouched.
///
/// Original: 0x00E6BEC0, cdecl, no arguments.
export!(cdecl, rw_00e6bec0() -> u32 {
    unsafe {
        const DST0: u32 = 0x171B9B0;
        const DST1: u32 = 0x171B9B4;
        const DST2: u32 = 0x171B9B8;
        let v0 = *global::<u32>(0x1050AB4);
        let v1 = *global::<u32>(0x1050AB8);
        *global::<u32>(DST0) = v0;
        *global::<u32>(DST1) = v0;
        *global::<u32>(DST2) = v1;
        0
    }
});
