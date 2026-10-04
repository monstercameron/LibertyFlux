// original: 0x00bb9f40 TASK_LEAVE_CAR_IN_DIRECTION
use lf_k2_rt::{callee_cdecl, export};
// TASK_LEAVE_CAR_IN_DIRECTION: forward (ped, vehicle, direction?) with the
// third argument coerced through the incoming stack slot.
// v2 port: the rewrite passes the bare 0/1 flag; the dead-slot high
// bytes are masked in the contract (call_skip).
export!(cdecl, rw_00BB9F40(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, *a.add(1), u32::from(*a.add(2) != 0))
    }
});
