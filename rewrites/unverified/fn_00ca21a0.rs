// original: 0x00CA21A0 GetPedScenarioEntityValue

/// Read the 4-bit scenario state from the word at object offset +0x28. State 3 returns the dword at +0x2C4, state 6 returns +0x118, and every other state returns zero. This stdcall helper removes its object-pointer argument.
lf_checker_rt::export!(stdcall, rw_00ca21a0(object: u32) -> u32 {
    const STATE: u32 = 0x28;
    const STATE_THREE_VALUE: u32 = 0x2C4;
    const STATE_SIX_VALUE: u32 = 0x118;
    let flags = unsafe { (object.wrapping_add(STATE) as *const u32).read_unaligned() };
    let scenario_state = (flags >> 6) & 0xF;
    unsafe {
        match scenario_state {
            3 => (object.wrapping_add(STATE_THREE_VALUE) as *const u32).read_unaligned(),
            6 => (object.wrapping_add(STATE_SIX_VALUE) as *const u32).read_unaligned(),
            _ => 0,
        }
    }
});
