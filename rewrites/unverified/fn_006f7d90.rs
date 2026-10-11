// original: 0x006F7D90 mainloop_timer_parse_descriptor

/// Measure the null-terminated text referenced by the input descriptor, build
/// the six-word local parse state, and pass it with the descriptor, text and
/// length to the parser helper. Then ask the validator helper about that state
/// and return its low byte. This proof covers the initialized-state path where
/// the parser leaves the local state empty; richer parse results are outside
/// the contract.
lf_checker_rt::export!(thiscall, rw_006f7d90(context: u32, input: u32) -> u8 {
    unsafe fn read32(address: u32) -> u32 { (address as *const u32).read_unaligned() }
    unsafe fn read8(address: u32) -> u8 { (address as *const u8).read() }

    let text = unsafe { read32(input + 8) };
    let mut text_len = 0u32;
    unsafe {
        while read8(text + text_len) != 0 {
            text_len = text_len.wrapping_add(1);
        }
    }

    let mut parse_state = [0u32, 0, u32::MAX, u32::MAX, u32::MAX, u32::MAX];
    let parse_state_ptr = parse_state.as_mut_ptr() as u32;
    let _parse_result: u32 = lf_checker_rt::callee_thiscall!(1, u32, parse_state_ptr, text, text.wrapping_add(1), text_len);
    lf_checker_rt::callee_thiscall!(2, u8, parse_state_ptr)
});
