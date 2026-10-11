// original: 0x00CA2150 GetPedScenarioStateValue

/// Read the 4-bit scenario state from the word at object offset +0x28. State 3 returns the dword at +0xB94, state 6 returns +0x124, and every other state returns 0xFFFFFFFF as a signed -1. This stdcall helper removes its object-pointer argument.
lf_checker_rt::export!(stdcall, rw_00ca2150(object: u32) -> u32 {
    const STATE: u32 = 0x28;
    const STATE_THREE_VALUE: u32 = 0xB94;
    const STATE_SIX_VALUE: u32 = 0x124;
    let flags = unsafe { (object.wrapping_add(STATE) as *const u32).read_unaligned() };
    let scenario_state = (flags >> 6) & 0xF;
    unsafe {
        match scenario_state {
            3 => (object.wrapping_add(STATE_THREE_VALUE) as *const u32).read_unaligned(),
            6 => (object.wrapping_add(STATE_SIX_VALUE) as *const u32).read_unaligned(),
            _ => u32::MAX,
        }
    }
});
