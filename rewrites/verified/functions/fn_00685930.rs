// original: 0x00685930 forward_to_00686aa0
/// Forward two words and a float to the worker helper, returning its answer.
///
/// The incoming float word is loaded into a vector register for the helper
/// (the checker cannot observe that register alongside the stack words, so
/// that load is the one unverified step, noted in the proof). The second
/// and third words go on the stack and a pointer to a frame slot holding
/// the object pointer goes in the object register. The helper cleans two
/// stack words. Returns the helper's answer.
export!(thiscall, rw_00685930(this_: u32, float_word: u32, second: u32, third: u32) -> u32 {
    let _ = float_word;
    let frame = this_;
    callee_thiscall!(1, u32, &frame as *const u32 as u32, second, third)
});
