// original: 0x00e61d10 register_teardown_05
use lf_checker_rt::{callee_cdecl, export, relocated};

// Rewrite of the original at 0x00e61d10 (`register_teardown_05`): Submit one teardown routine to the init/teardown registry and return its status.
//
// Takes no arguments; returns whatever the registry helper returns.
export!(cdecl, rw_00e61d10() -> u32 {
    callee_cdecl!(0, u32, relocated(0x00E706F0))
});
