// original: 0x009A6820 audio_set_flag_2d (proposed)

/// Stores the low byte of its argument into audio flag byte 0x2D.
///
/// stdcall, one stack word; only the low byte is read. Leaf: no calls,
/// no reads. The return register keeps the caller's upper bytes, so the
/// contract compares no return channel.
lf_checker_rt::export!(stdcall, rw_009a6820(value: u32) -> u32 {
    unsafe {
        lf_checker_rt::global::<u8>(0x00116252D).write(value as u8);
    }
    0
});
