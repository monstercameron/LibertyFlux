// original: 0x00bb2770 PLAYER_HAS_CHAR
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `PLAYER_HAS_CHAR`: calls its engine function and writes the
/// low byte of the result (zero-extended) to the return slot.
export!(cdecl, rw_00bb2770(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        let slot = *ctx as *mut u32;
        let answer = callee_cdecl!(1, u32, *args.add(0));
        *slot = answer & 0xFF;
        0
    }
});
