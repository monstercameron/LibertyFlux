// original: 0x00bd84d0 NETWORK_GET_NUM_PLAYERS_MET
use lf_k2_rt::{callee_cdecl, export};
// NETWORK_GET_NUM_PLAYERS_MET: take no arguments; store the full answer
// through the return slot and return it.
export!(cdecl, rw_00BD84D0(ctx: u32) -> u32 {
    unsafe {
        let n = callee_cdecl!(1, u32,);
        *ret_slot(ctx) = n;
        n
    }
});
