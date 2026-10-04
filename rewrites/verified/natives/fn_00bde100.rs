// original: 0x00bde100 GET_IS_DEPOSIT_ANIM_RUNNING
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `GET_IS_DEPOSIT_ANIM_RUNNING`: takes no arguments and stores the low byte of the engine answer.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
export!(cdecl, rn24_get_is_deposit_anim_running(ctx: u32) -> () {
        let answer: u32 = callee_cdecl!(1, u32,);
        let slot = unsafe { *(ctx as *const u32) as *mut u32 };
        // Only the low byte is kept (movzx after the call).
        unsafe { slot.write(answer & 0xFF) };
});
