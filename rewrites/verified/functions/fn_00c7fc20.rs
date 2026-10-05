// original: 0x00c7fc20 CTaskComplexChatScenario::vf6

/// Scenario blend weight for the chat scenario: 25.0 when active, -1.0 else.
///
/// Returns 25.0 when the flag byte at `this+0x20` is clear or the word at
/// `this+0x1c` is non-zero (the same predicate as the neighbouring active-flag
/// checks), else -1.0. The original stages the chosen bit pattern on its own
/// stack slot and loads it with `fld`; the value is returned in ST0. The one
/// stack word is popped but never read.
///
/// Original: thiscall, one stack word (the callee pops 4 bytes), float result in ST0.
lf_checker_rt::export!(thiscall, rw_00c7fc20(this: u32, _u: u32) -> f32 {
    unsafe {
        const FLAG_OFF: u32 = 0x20;
        const STATE_OFF: u32 = 0x1c;
        const ACTIVE_W: f32 = 25.0;
        const IDLE_W: f32 = -1.0;
        let flag = ((this + FLAG_OFF) as *const u8).read();
        let state = ((this + STATE_OFF) as *const u32).read_unaligned();
        if flag == 0 || state != 0 { ACTIVE_W } else { IDLE_W }
    }
});
