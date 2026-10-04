// original: 0x00b875f0 SET_CAM_PROPAGATE
use lf_k2_rt::{callee_cdecl, export};
// SET_CAM_PROPAGATE: forward (cam, propagate?) with the bool coerced through
// the incoming stack slot as above.
// v2 port: the rewrite passes the bare 0/1 flag; the dead-slot high
// bytes are masked in the contract (call_skip).
export!(cdecl, rw_00B875F0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, u32::from(*a.add(1) != 0))
    }
});
