// original: 0x00bb8d80 IS_SITTING_OBJECT_NEAR
use lf_k2_rt::{callee_cdecl, export};
/// Report whether a sitting object is near a position: pass the four
/// script words (three position floats plus a radius, all bitwise) to
/// the engine and store the low byte of its answer (zero-extended) in
/// the return slot.
export!(cdecl, rw_00bb8d80(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let ans: u32 =
            callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3));
        *(*ctx as *mut u32) = ans & 0xFF;
        0
    }
});
