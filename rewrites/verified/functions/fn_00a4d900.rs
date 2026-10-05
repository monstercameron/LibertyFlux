// original: 0x00A4D900 vehicle_set_mode_10b8 (proposed)

/// Switches the mode byte at `this + MODE`, signalling twice, and runs a
/// sub-object cycle when the new mode is 2.
///
/// Reads `cur = byte[this + MODE]` (0x10B8, zero-extended) and returns at once
/// when it equals the full argument word. Otherwise signals the first callee
/// twice (cdecl, `(this, 1)` then `(this, 0)`), storing the argument's low
/// byte into the mode between the two calls. When the stored byte is 2,
/// signals the second callee (thiscall, `this + SUB` (0x80) in `ecx`, one
/// word of 1) and then the third (cdecl, `this + SUB`). Returns nothing
/// defined.
///
/// Original: 0x00A4D900 (thiscall, one stack word), three callees.
lf_checker_rt::export!(thiscall, rw_00A4D900(this: u32, arg: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x10B8;
        const SUB: u32 = 0x80;
        const CYCLE_MODE: u8 = 2;
        const SIGNAL_CALLEE: u32 = 1;
        const START_CALLEE: u32 = 2;
        const FINISH_CALLEE: u32 = 3;
        let mode = (this + MODE) as *mut u8;
        if mode.read() as u32 == arg {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(SIGNAL_CALLEE, u32, this, 1);
        mode.write((arg & 0xFF) as u8);
        lf_checker_rt::callee_cdecl!(SIGNAL_CALLEE, u32, this, 0);
        if mode.read() != CYCLE_MODE {
            return 0;
        }
        let sub = this + SUB;
        lf_checker_rt::callee_thiscall!(START_CALLEE, u32, sub, 1);
        lf_checker_rt::callee_cdecl!(FINISH_CALLEE, u32, sub);
        0
    }
});
