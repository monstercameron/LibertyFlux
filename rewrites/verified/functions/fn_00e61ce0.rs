// original: 0x00e61ce0 init_array100_and_register
use lf_checker_rt::{callee_cdecl, export, relocated};

// Rewrite of the original at 0x00e61ce0 (`init_array100_and_register`): Construct 100 objects in a static array via the element constructor, submit the teardown routine, return the status.
//
// Takes no arguments; returns whatever the registry helper returns.
export!(cdecl, rw_00e61ce0() -> u32 {
    const FIRST_OBJECT: u32 = 0x1A0B0F8;
    const STRIDE: u32 = 0x31F0;
    const COUNT: u32 = 100;
    for i in 0..COUNT {
        let obj = relocated(FIRST_OBJECT).wrapping_add(i.wrapping_mul(STRIDE));
        let _status: u32 = lf_checker_rt::callee_thiscall!(1, u32, obj);
    }
    callee_cdecl!(0, u32, relocated(0x00E706C0))
});
