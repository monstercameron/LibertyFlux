// original: 0x00bb64a0 PLAYSTATS_FLOAT
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `PLAYSTATS_FLOAT`.
///
/// Reports a floating point statistic to the play-stats backend.
///
/// Handler mechanics: takes the native call context,
/// Forwards the stat id and the value bits (bitwise) to the stat
/// packet builder.
export!(cdecl, rw_00bb64a0(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let id = unsafe { *args };
    let value = unsafe { *args.add(1) };
    callee_cdecl!(1, u32, id, value);
});
