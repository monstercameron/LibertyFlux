// original: 0x00E5BDF0 broadcast_float_to_18d2190_x3
/// Broadcasts one single-precision global to 3 fixed global slots.
///
/// Behaviour: reads the source word once and stores its exact bits to
/// every destination (the original moves raw 32-bit lanes, so NaN
/// payloads and signed zeros pass through untouched).
lf_checker_rt::export!(cdecl, rs188_00e5bdf0() -> u32 {
    /// Source and destination file VAs.
    const SRC: u32 = 0x017AD148;
    const DST0: u32 = 0x018D2190;
    const DST1: u32 = 0x018D2194;
    const DST2: u32 = 0x018D2198;
    unsafe {
        let v = (lf_checker_rt::global::<u32>(SRC) as *const u32).read();
        (lf_checker_rt::global::<u32>(DST0) as *mut u32).write(v);
        (lf_checker_rt::global::<u32>(DST1) as *mut u32).write(v);
        (lf_checker_rt::global::<u32>(DST2) as *mut u32).write(v);
    }
    // The original returns nothing meaningful (eax is preserved); the
    // contract compares no return channel.
    0
});
