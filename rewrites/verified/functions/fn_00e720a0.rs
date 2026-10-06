// original: 0x00e720a0 audio_cond_free_20a0
/// Release the heap block at 0x012b6520 when the flag word at 0x012b6526 is nonzero.
///
/// Reads the 16-bit flag word and compares it against zero with an
/// equality test (no signedness involved); when set, pushes the pointer
/// from the global slot and calls the release callee (cdecl/1, callee
/// id 1), discarding its answer. Takes no arguments (cdecl/0); EAX on
/// return is the callee's answer on the taken path and the untouched
/// incoming EAX otherwise, so the contract compares no return channel.
export!(cdecl, rw_00e720a0() -> u32 {
    unsafe {
        const PTR_SLOT: u32 = 0x012b6520;
        const FLAG: u32 = 0x012b6526;
        let flag = lf_checker_rt::global::<u16>(FLAG).read_unaligned();
        if flag != 0 {
            let p = lf_checker_rt::global::<u32>(PTR_SLOT).read_unaligned();
            lf_checker_rt::callee_cdecl!(1, u32, p);
        }
        0
    }
});
