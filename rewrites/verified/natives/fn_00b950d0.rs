// original: 0x00b950d0 SET_UP_TRIP_SKIP_FOR_SPECIFIC_VEHICLE
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `SET_UP_TRIP_SKIP_FOR_SPECIFIC_VEHICLE`.
///
/// Sets up a skippable trip for a specific vehicle.
///
/// Handler mechanics: takes the native call context,
/// Forwards four coordinates (bitwise) and the vehicle handle to the
/// trip-skip worker.
export!(cdecl, rw_00b950d0(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let f0 = unsafe { *args };
    let f1 = unsafe { *args.add(1) };
    let f2 = unsafe { *args.add(2) };
    let f3 = unsafe { *args.add(3) };
    let veh = unsafe { *args.add(4) };
    callee_cdecl!(1, u32, f0, f1, f2, f3, veh);
});
