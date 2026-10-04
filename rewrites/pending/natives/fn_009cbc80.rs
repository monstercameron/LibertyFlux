// original: 0x009cbc80 GET_SOUND_ID
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `GET_SOUND_ID`: takes no arguments and stores the full engine answer.
///
/// `ctx` points at the script call context: dword 0 is the return slot,
/// dword 2 is the script argument array. The engine call is intercepted
/// by the checker, which compares its arguments.
export!(cdecl, rn24_get_sound_id(ctx: u32) -> () {
        let answer: u32 = callee_cdecl!(1, u32,);
        let slot = unsafe { *(ctx as *const u32) as *mut u32 };
        unsafe { slot.write(answer) };
});
