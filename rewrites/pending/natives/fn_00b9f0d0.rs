// original: 0x00b9f0d0 GET_CHAR_WILL_COWER_INSTEAD_OF_FLEEING
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `GET_CHAR_WILL_COWER_INSTEAD_OF_FLEEING`: forwards one script handle to the engine and stores the low byte of the answer.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
export!(cdecl, rn24_get_char_will_cower_instead_of_fleeing(ctx: u32) -> () {
        let args = unsafe { (*(ctx as *const u32).add(2)) as *const u32 };
        let answer: u32 = callee_cdecl!(1, u32, unsafe { args.read() });
        let slot = unsafe { *(ctx as *const u32) as *mut u32 };
        // Only the low byte is kept (movzx after the call).
        unsafe { slot.write(answer & 0xFF) };
});
