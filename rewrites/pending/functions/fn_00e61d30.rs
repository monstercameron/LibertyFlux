// original: 0x00e61d30 init_critsec_and_register_00
use lf_checker_rt::{callee_cdecl, export, relocated};

// Rewrite of the original at 0x00e61d30 (`init_critsec_and_register_00`): Initialize one critical section, submit that subsystem teardown routine, return the status.
//
// Takes no arguments; returns whatever the registry helper returns.
export!(cdecl, rw_00e61d30() -> u32 {
    let _cs_init: u32 = lf_checker_rt::callee_stdcall!(1, u32, relocated(0x1B484A0));
    callee_cdecl!(0, u32, relocated(0x00E70750))
});
