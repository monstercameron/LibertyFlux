// original: 0x00a96a30 fade_ratio_push
/// Measures the channel ratio and hands the default sample to the sink.
///
/// The measured level (returned on the x87 stack) is popped and discarded;
/// the sink always receives a pointer to two one-half words built in the
/// caller's frame. Returns whatever the sink answers.
export!(thiscall, rw_00a96a30(this: u32) -> u32 {
    unsafe {
        let _level: f32 = callee_thiscall!(1, f32, this);
        let sample = [0.5f32, 0.5f32];
        callee_cdecl!(2, u32, sample.as_ptr() as u32)
    }
});
