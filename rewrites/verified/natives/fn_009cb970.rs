// original: 0x009cb970 ADD_NEW_CONVERSATION_SPEAKER
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `ADD_NEW_CONVERSATION_SPEAKER`: forwards script args [arg0 (dword), arg1 (dword), arg2 (dword)]
/// to its engine function and returns nothing (void).
export!(cdecl, rw_009cb970(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2));
        0
    }
});
