// original: 0x00bd7c40 GET_SAFE_LOCAL_RESTART_COORDS
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `GET_SAFE_LOCAL_RESTART_COORDS`.
///
/// Finds safe restart coordinates near a position.
///
/// Handler mechanics: takes the native call context,
/// Forwards three coordinates (bitwise) plus three integer arguments to
/// the safe-ground search worker.
export!(cdecl, rw_00bd7c40(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let f0 = unsafe { *args };
    let f1 = unsafe { *args.add(1) };
    let f2 = unsafe { *args.add(2) };
    let a3 = unsafe { *args.add(3) };
    let a4 = unsafe { *args.add(4) };
    let a5 = unsafe { *args.add(5) };
    callee_cdecl!(1, u32, f0, f1, f2, a3, a4, a5);
});
