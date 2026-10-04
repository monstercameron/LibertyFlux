// original: 0x00bd8ab0 NETWORK_LIMIT_TO_16_PLAYERS
use lf_k2_rt::{callee_cdecl, export};
/// Report the 16-player network limit state: call the engine with no
/// arguments and store the low byte of its answer (zero-extended) in
/// the return slot.
export!(cdecl, rw_00bd8ab0(ctx: *const u32) -> u32 {
    unsafe {
        let ans: u32 = callee_cdecl!(1, u32,);
        *(*ctx as *mut u32) = ans & 0xFF;
        0
    }
});
