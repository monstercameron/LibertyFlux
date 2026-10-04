// original: 0x00E5BF20 init_timer_float_block
/// Initialises the four-word floating-point state block in place.
///
/// Behaviour: stamps four constant words: a NaN-payload/all-ones word,
/// a quiet-NaN word, a positive-infinity word and a zero word. The other
/// broadcast routines in this batch read three of these words back.
lf_checker_rt::export!(cdecl, rs188_00e5bf20() -> u32 {
    /// File VA of the first word of the state block.
    const BASE: u32 = 0x017AD148;
    unsafe {
        let b = lf_checker_rt::global::<u32>(BASE) as *mut u32;
        b.add(3).write(0x00000000); // +0xC
        b.add(0).write(0xFFFFFFFF); // +0x0
        b.add(2).write(0x7F800000); // +0x8
        b.add(1).write(0x7FC00000); // +0x4
    }
    // The original returns nothing meaningful (eax is preserved); the
    // contract compares no return channel.
    0
});
