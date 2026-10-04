// original: 0x00E5BEF0 broadcast_float_to_18d2180_x3
/// Broadcasts one single-precision global to 3 fixed global slots.
///
/// Behaviour: reads the source word once and stores its exact bits to
/// every destination (the original moves raw 32-bit lanes, so NaN
/// payloads and signed zeros pass through untouched).
lf_checker_rt::export!(cdecl, rs188_00e5bef0() -> u32 {
    /// Source and destination file VAs.
    const SRC: u32 = 0x017AD14C;
    const DST0: u32 = 0x018D2180;
    const DST1: u32 = 0x018D2184;
    const DST2: u32 = 0x018D2188;
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
