// original: 0x00E5BEA0 broadcast_float4_to_18d21b0
/// Broadcasts one single-precision global to 4 fixed global slots.
///
/// Behaviour: reads the source word once and stores its exact bits to
/// every destination (the original moves raw 32-bit lanes, so NaN
/// payloads and signed zeros pass through untouched).
lf_checker_rt::export!(cdecl, rs188_00e5bea0() -> u32 {
    /// Source and destination file VAs.
    const SRC: u32 = 0x017AD148;
    const DST0: u32 = 0x018D21B0;
    const DST1: u32 = 0x018D21B4;
    const DST2: u32 = 0x018D21B8;
    const DST3: u32 = 0x018D21BC;
    unsafe {
        let v = (lf_checker_rt::global::<u32>(SRC) as *const u32).read();
        (lf_checker_rt::global::<u32>(DST0) as *mut u32).write(v);
        (lf_checker_rt::global::<u32>(DST1) as *mut u32).write(v);
        (lf_checker_rt::global::<u32>(DST2) as *mut u32).write(v);
        (lf_checker_rt::global::<u32>(DST3) as *mut u32).write(v);
    }
    // The original returns nothing meaningful (eax is preserved); the
    // contract compares no return channel.
    0
});
