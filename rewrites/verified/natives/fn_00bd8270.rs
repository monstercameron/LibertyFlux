// original: 0x00bd8270 NETWORK_END_SESSION_PENDING
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `NETWORK_END_SESSION_PENDING`: takes no arguments and stores the low byte of the engine answer.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
export!(cdecl, rn24_network_end_session_pending(ctx: u32) -> () {
        let answer: u32 = callee_cdecl!(1, u32,);
        let slot = unsafe { *(ctx as *const u32) as *mut u32 };
        // Only the low byte is kept (movzx after the call).
        unsafe { slot.write(answer & 0xFF) };
});
