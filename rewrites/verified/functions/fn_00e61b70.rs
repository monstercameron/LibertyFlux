// original: 0x00e61b70 register_teardown_02
use lf_checker_rt::{callee_cdecl, export, relocated};

// Rewrite of the original at 0x00e61b70 (`register_teardown_02`): Submit one teardown routine to the init/teardown registry and return its status.
//
// Takes no arguments; returns whatever the registry helper returns.
export!(cdecl, rw_00e61b70() -> u32 {
    callee_cdecl!(0, u32, relocated(0x00E70580))
});
