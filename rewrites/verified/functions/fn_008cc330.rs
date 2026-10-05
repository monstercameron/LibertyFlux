// original: 0x008CC330 stream_drive_state (proposed)

/// Drives the streaming state machine one step. When `reset` is non-zero the
/// state is cleared to 0, the counter zeroed, the active byte set and the
/// mode byte `MODE` set to `mode`, then state 0 runs. Otherwise the current
/// state runs (a state above 3 returns 1 at once):
/// state 0 calls the setup callee (callee 0, cdecl with the mode byte and
/// the `detail` flag bit), advances to state 1 and shares state 1's probe;
/// state 1 calls the probe callee (callee 1): 2 with the magic word equal to
/// `MAGIC` finishes through the finish path (active byte cleared, finish
/// callee called, returns 2), 2 otherwise parks in state 3 and returns 1,
/// 0 advances to state 2, runs the session callee (callee 2) and returns 1,
/// anything else returns 1; state 2 calls the check callee (callee 3, cdecl
/// with 0): 2 parks in state 3 and returns 1, non-zero returns 1, 0 clears
/// the active byte, runs the finish callee (callee 5) and returns 0; state 3
/// calls the ready callee (callee 4): a zero low byte returns 1, otherwise
/// the finish path runs and returns 2.
///
/// The original dispatches through a jump table; the rewrite is the
/// equivalent match. Three stack arguments (cdecl); the step result is in
/// EAX.
lf_checker_rt::export!(cdecl, rw_008CC330(reset: u32, detail: u32, mode: u32) -> u32 {
    unsafe {
        /// Setup callee id.
        const SETUP: u32 = 0;
        /// Probe callee id.
        const PROBE: u32 = 1;
        /// Session callee id.
        const SESSION: u32 = 2;
        /// Check callee id.
        const CHECK: u32 = 3;
        /// Ready callee id.
        const READY: u32 = 4;
        /// Finish callee id.
        const FINISH: u32 = 5;
        /// Global state word.
        const STATE: u32 = 0x1173368;
        /// Global counter word.
        const COUNTER: u32 = 0x1172CCC;
        /// Global active byte.
        const ACTIVE: u32 = 0x1172CD0;
        /// Global mode byte.
        const MODE: u32 = 0x117336C;
        /// Global magic word.
        const MAGIC_AT: u32 = 0x1031BAC;
        /// Magic value selecting the finish path.
        const MAGIC: u32 = 0x21;
        let state_reg = lf_checker_rt::relocated(STATE);
        let mode_reg = lf_checker_rt::relocated(MODE);
        let active_reg = lf_checker_rt::relocated(ACTIVE);
        let (state, mode_b): (u32, u32);
        if reset & 0xFF != 0 {
            (state_reg as *mut u32).write(0);
            (lf_checker_rt::relocated(COUNTER) as *mut u32).write(0);
            (active_reg as *mut u8).write(1);
            (mode_reg as *mut u8).write((mode & 0xFF) as u8);
            state = 0;
            mode_b = mode & 0xFF;
        } else {
            state = (state_reg as *const u32).read();
            if state > 3 {
                return 1;
            }
            mode_b = (mode_reg as *const u8).read() as u32;
        }
        // Shared probe tail of states 0 and 1; returns Some when decided.
        unsafe fn probe_tail(state_reg: u32, active_reg: u32) -> Option<u32> {
            unsafe {
                const PROBE: u32 = 1;
                const SESSION: u32 = 2;
                const FINISH: u32 = 5;
                const STATE: u32 = 0x1173368;
                const ACTIVE: u32 = 0x1172CD0;
                const MAGIC_AT: u32 = 0x1031BAC;
                const MAGIC: u32 = 0x21;
                let probe: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32,);
                if probe == 2 {
                    if (lf_checker_rt::relocated(MAGIC_AT) as *const u32).read() == MAGIC {
                        (active_reg as *mut u8).write(0);
                        let _f: u32 = lf_checker_rt::callee_cdecl!(FINISH, u32,);
                        return Some(2);
                    }
                    (state_reg as *mut u32).write(3);
                    return Some(1);
                }
                if probe != 0 {
                    return Some(1);
                }
                (state_reg as *mut u32).write(2);
                let _s: u32 = lf_checker_rt::callee_cdecl!(SESSION, u32,);
                Some(1)
            }
        }
        match state {
            0 => {
                let flag = u32::from(detail & 0xFF != 0);
                let _s: u32 = lf_checker_rt::callee_cdecl!(SETUP, u32, flag, mode_b);
                (state_reg as *mut u32).write(1);
                probe_tail(state_reg, active_reg).unwrap_or(1)
            }
            1 => probe_tail(state_reg, active_reg).unwrap_or(1),
            2 => {
                let chk: u32 = lf_checker_rt::callee_cdecl!(CHECK, u32, 0);
                if chk == 2 {
                    (state_reg as *mut u32).write(3);
                    return 1;
                }
                if chk != 0 {
                    return 1;
                }
                (active_reg as *mut u8).write(0);
                let _f: u32 = lf_checker_rt::callee_cdecl!(FINISH, u32,);
                0
            }
            _ => {
                let ready: u32 = lf_checker_rt::callee_cdecl!(READY, u32,);
                if ready & 0xFF == 0 {
                    return 1;
                }
                (active_reg as *mut u8).write(0);
                let _f: u32 = lf_checker_rt::callee_cdecl!(FINISH, u32,);
                2
            }
        }
    }
});
