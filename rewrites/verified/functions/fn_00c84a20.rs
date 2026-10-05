// original: 0x00c84a20 scenario_active_flag (proposed)

/// Report whether the scenario object is in its active state.
///
/// Returns 1 when the flag byte at `this+0x20` is clear or the word at
/// `this+0x1c` is non-zero, else 0. Only AL is set (AH keeps its entry value,
/// so the contract compares `al`). No memory is written.
///
/// Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00c84a20(this: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x20;
        const STATE_OFF: u32 = 0x1c;
        let flag = ((this + FLAG_OFF) as *const u8).read();
        let state = ((this + STATE_OFF) as *const u32).read_unaligned();
        u32::from(flag == 0 || state != 0)
    }
});
