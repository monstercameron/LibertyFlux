// original: 0x00ba17a0 SET_CHAR_MONEY
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `SET_CHAR_MONEY`: forwards script args [arg0 (dword), arg1 (dword)]
/// to its engine function and returns nothing (void).
export!(cdecl, rw_00ba17a0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args.add(0), *args.add(1));
        0
    }
});
