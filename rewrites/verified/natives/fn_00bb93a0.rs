// original: 0x00bb93a0 TASK_CHAR_SLIDE_TO_COORD_AND_PLAY_ANIM_HDG_RATE
use lf_k2_rt::{callee_cdecl, export, relocated};
/// Native handler `TASK_CHAR_SLIDE_TO_COORD_AND_PLAY_ANIM_HDG_RATE`.
///
/// Slides a character to a coordinate while playing an animation.
///
/// Handler mechanics: takes the native call context,
/// Passes its call context untouched to the shared task-argument adapter
/// together with the engine function address.
export!(cdecl, rw_00bb93a0(ctx: u32) -> () {
    callee_cdecl!(1, u32, relocated(0x00BBD270), ctx);
});
