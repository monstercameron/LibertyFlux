// original: 0x00E5BD20 copy_float3_to_18d2170
/// Copies 3 single-precision values between fixed global slots.
///
/// Behaviour: Lane-by-lane copy, one load and one store per value,
/// preserving every bit (the original moves raw 32-bit lanes, so NaN
/// payloads and signed zeros pass through untouched).
lf_checker_rt::export!(cdecl, rs188_00e5bd20() -> u32 {
    /// First source and destination file VAs; lanes follow every 4 bytes.
    const SRC: u32 = 0x0110DB50;
    const DST: u32 = 0x018D2170;
    unsafe {
        let src = lf_checker_rt::global::<u32>(SRC) as *const u32;
        let dst = lf_checker_rt::global::<u32>(DST) as *mut u32;
        dst.write(src.read());
        dst.add(1).write(src.add(1).read());
        dst.add(2).write(src.add(2).read());
    }
    // The original returns nothing meaningful (eax is preserved); the
    // contract compares no return channel.
    0
});
