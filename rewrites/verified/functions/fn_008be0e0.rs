// original: 0x008BE0E0 input_mode_set (proposed)

/// Store the new input-mode id into the global mode slot.
///
/// The single stack argument is the mode id, written to the global word.
/// The return value is the argument with its low byte forced to 1: the
/// original loads the argument into eax and then sets only al, so the upper
/// three bytes of the argument survive in the result (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_008BE0E0(value: u32) -> u32 {
    unsafe {
        /// Global word holding the current input-mode id.
        const MODE_SLOT: u32 = 0x01160C24;
        lf_checker_rt::global::<u32>(MODE_SLOT).write_unaligned(value);
        (value & 0xFFFF_FF00) | 1
    }
});
