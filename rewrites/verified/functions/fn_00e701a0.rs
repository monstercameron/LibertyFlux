// original: 0x00E701A0 timer_gated_notify
/// Notify the timer target once when the notify gate is open.
///
/// When the gate flag is set, calls the engine notify hook with the timer
/// target (or a null target when the mode byte selects it), then closes the
/// gate, marks the argument block done and clears the result word.
export!(cdecl, rw_00e701a0() -> u32 {
    unsafe {
        const GATE: u32 = 0x01A01444;
        const MODE: u32 = 0x018B8309;
        const TARGET: u32 = 0x019F3A10;
        const ARG_BLOCK: u32 = 0x01A00E60;
        const RESULT: u32 = 0x01A01440;
        if *global::<u8>(GATE) & 1 == 0 {
            return 0;
        }
        let this = if *global::<u8>(MODE) == 0 {
            0
        } else {
            relocated(TARGET)
        };
        callee_thiscall!(1, u32, this, relocated(ARG_BLOCK));
        *global::<u8>(GATE) &= 0xFC;
        *global::<u32>(ARG_BLOCK) = 0xFFFF_FFFF;
        *global::<u32>(RESULT) = 0;
        0
    }
});
