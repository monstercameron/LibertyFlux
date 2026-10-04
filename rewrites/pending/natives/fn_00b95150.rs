// original: 0x00b95150 SET_UP_TRIP_SKIP_TO_BE_FINISHED_BY_SCRIPT
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `SET_UP_TRIP_SKIP_TO_BE_FINISHED_BY_SCRIPT`.
///
/// Sets up a skippable trip that the script finishes itself.
///
/// Handler mechanics: takes the native call context,
/// Forwards four coordinates (bitwise) to the trip-skip worker.
export!(cdecl, rw_00b95150(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let f0 = unsafe { *args };
    let f1 = unsafe { *args.add(1) };
    let f2 = unsafe { *args.add(2) };
    let f3 = unsafe { *args.add(3) };
    callee_cdecl!(1, u32, f0, f1, f2, f3);
});
