// original: 0x009a4730 audRadioAudioEntity::vf2
/// Original 0x009a4730 `audRadioAudioEntity::vf2`: refresh then tail-dispatch.
///
/// Runs the refresh helper on `this`, then tail-calls the shared dispatch
/// routine and returns its answer.
export!(thiscall, rw_009a4730(this_: u32) -> u32 {
    callee_thiscall!(1, u32, this_);
    callee_thiscall!(2, u32, this_)
});
