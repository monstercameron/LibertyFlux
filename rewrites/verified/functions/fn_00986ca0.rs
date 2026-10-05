// original: 0x00986CA0 aud_set_global_flag_inverted (proposed)

/// Global toggle of the audio subsystem: records whether the argument is zero.
///
/// Writes 1 to the global flag byte `FLAG` when the low byte of `value` is
/// zero, else 0. No meaningful return value.
/// Original: stdcall, one stack word, callee pops 4.
lf_checker_rt::export!(stdcall, rw_00986CA0(value: u32) -> u32 {
    const FLAG: u32 = 0x1238955;
    unsafe {
        *lf_checker_rt::global::<u8>(FLAG) = ((value as u8) == 0) as u8;
    }
    0
});
