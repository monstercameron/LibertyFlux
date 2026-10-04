// original: 0x005e7eb0 CREATE_HTML_VIEWPORT
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `CREATE_HTML_VIEWPORT`: forwards script args [arg0 (dword)]
/// to its engine function and returns nothing (void).
export!(cdecl, rw_005e7eb0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args.add(0));
        0
    }
});
