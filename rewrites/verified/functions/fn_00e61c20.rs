// original: 0x00e61c20 init_singleton_and_register_05
use lf_checker_rt::{callee_cdecl, export, relocated};

// Rewrite of the original at 0x00e61c20 (`init_singleton_and_register_05`): Run one singleton initializer, submit that subsystem teardown routine, return the status.
//
// Takes no arguments; returns whatever the registry helper returns.
export!(cdecl, rw_00e61c20() -> u32 {
    let _init_status: u32 = callee_cdecl!(1, u32,);
    callee_cdecl!(0, u32, relocated(0x00E70600))
});
