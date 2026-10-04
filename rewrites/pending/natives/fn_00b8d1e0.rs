// original: 0x00b8d1e0 PRINT_WITH_3_NUMBERS_NOW
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `PRINT_WITH_3_NUMBERS_NOW`: forwards script args [arg0 (dword), arg1 (dword), arg2 (dword), arg3 (dword), arg4 (dword), arg5 (dword)]
/// to its engine function and returns nothing (void).
export!(cdecl, rw_00b8d1e0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5));
        0
    }
});
