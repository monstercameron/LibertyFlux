// original: 0x00E5BE20 store_float_to_1b4b330
/// Copies one single-precision global to a fixed global slot.
///
/// Behaviour: loads the source word and stores its exact bits
/// (the original moves a raw 32-bit lane, so NaN payloads and
/// signed zeros pass through untouched).
lf_checker_rt::export!(cdecl, rs188_00e5be20() -> u32 {
    /// Source and destination file VAs.
    const SRC: u32 = 0x017AD148;
    const DST0: u32 = 0x01B4B330;
    unsafe {
        let v = (lf_checker_rt::global::<u32>(SRC) as *const u32).read();
        (lf_checker_rt::global::<u32>(DST0) as *mut u32).write(v);
    }
    // The original returns nothing meaningful (eax is preserved); the
    // contract compares no return channel.
    0
});
