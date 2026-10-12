// original: 0x005BF710 hint_flag_raise_and_notify

/// Set requested hint-state bits and notify the shared state helper.
///
/// The cdecl stack argument is ORed into the relocated hint-flags global, then
/// the helper is called with a zero word and its return value is forwarded.
/// The original also clears its incoming argument slot before the tail call;
/// the scripted call log observes that forwarded zero.
lf_checker_rt::export!(cdecl, rw_005bf710(flags: u32) -> u32 {
    const HINT_FLAGS: u32 = 0x018B6ED8;
    const HELPER_ARGUMENT: u32 = 0;

    unsafe {
        let flags_global = lf_checker_rt::global::<u32>(HINT_FLAGS);
        let current_flags = flags_global.read_unaligned();
        flags_global.write_unaligned(current_flags | flags);
    }
    lf_checker_rt::callee_cdecl!(1, u32, HELPER_ARGUMENT)
});
