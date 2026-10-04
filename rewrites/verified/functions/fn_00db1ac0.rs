// original: 0x00db1ac0 process_ui_state_change
/// Service one round of the global UI state machine.
///
/// When the state word reads idle-busy it is reset and the round ends.
/// Otherwise a two-word scratch snapshot is captured, and when the state
/// still matches its shadow the probe runs: a quiet probe triggers the
/// shared follow-up, then the snapshot is always torn down. The result is
/// the teardown's result on the full paths; the early path leaves the
/// entry garbage in place, so callers must ignore the result there.
export!(cdecl, rw_00db1ac0() -> u32 {
    unsafe {
        const STATE: u32 = 0x017a65a8;
        const SHADOW: u32 = 0x017a65ac;
        const SOURCE: u32 = 0x017a65b4;
        const TARGET: u32 = 0x011737d0;
        const IDLE_BUSY: u32 = 3;

        if *global::<u32>(STATE) == IDLE_BUSY {
            *global::<u32>(STATE) = 0;
            return IDLE_BUSY;
        }
        let mut frame: [u32; 2] = [0, 0];
        let words = frame.as_mut_ptr();
        callee_thiscall!(1, u32, words.add(1) as u32, relocated(SOURCE));
        if *global::<u32>(STATE) != *global::<u32>(SHADOW) {
            return callee_thiscall!(5, u32, words as u32);
        }
        callee_thiscall!(2, u32, words as u32);
        let probe = callee_cdecl!(3, u32,);
        if (probe as u8) == 0 {
            callee_thiscall!(4, u32, relocated(TARGET), 2);
        }
        callee_thiscall!(5, u32, words as u32)
    }
});
