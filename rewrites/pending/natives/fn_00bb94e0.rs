// original: 0x00BB94E0 TASK_COMBAT_HATED_TARGETS_AROUND_CHAR_TIMED
use lf_k2_rt::{callee_cdecl, export, relocated};

/// TASK_COMBAT_HATED_TARGETS_AROUND_CHAR_TIMED: timed combat task.
///
/// Native handler. Forwards ped handle, radius and duration to the task engine. Floats are bit-copied.
export!(cdecl, rw_00bb94e0(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1); // bit-copied f32
        let a2 = *args.add(2); // bit-copied f32
        callee_cdecl!(1, u32, a0, a1, a2)
    }
});
