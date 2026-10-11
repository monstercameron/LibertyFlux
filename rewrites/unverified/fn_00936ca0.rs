// original: 0x00936CA0 clear_stale_network_state_generation

/// Clear the receiver-independent network state byte only when the state is
/// outside the three preserved values and the requested generation matches
/// the current generation global. The state and generation are unsigned
/// 32-bit words compared for equality. This cdecl routine takes one stack
/// argument and has no defined return value.
lf_checker_rt::export!(cdecl, rw_00936ca0(requested_generation: u32) -> () {
    unsafe {
        const STATE_GLOBAL: u32 = 0x011a_4dd4;
        const GENERATION_GLOBAL: u32 = 0x011a_4ddc;
        let state = unsafe { lf_checker_rt::global::<u32>(STATE_GLOBAL).read_unaligned() };
        if state == 0 || state == 5 || state == 4 {
            return;
        }
        let generation = unsafe { lf_checker_rt::global::<u32>(GENERATION_GLOBAL).read_unaligned() };
        let next_state = if generation == requested_generation { 0 } else { state };
        unsafe { lf_checker_rt::global::<u32>(STATE_GLOBAL).write_unaligned(next_state) };
    }
});
