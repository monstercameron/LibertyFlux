// original: 0x00bd8800 NETWORK_IS_INVITEE_ONLINE
use lf_k2_rt::{callee_cdecl, export};
// NETWORK_IS_INVITEE_ONLINE: take no arguments; store the answer's low
// byte through the return slot. Returns the slot.
export!(cdecl, rw_00BD8800(ctx: u32) -> u32 {
    unsafe {
        let online = __lf_callee_cdecl!(1, u32,) & 0xFF;
        let slot = ret_slot(ctx);
        *slot = online;
        slot as u32
    }
});

// Shared context helpers, verbatim from lane r-n20 (its out/ files use them).
unsafe fn ret_slot(ctx: u32) -> *mut u32 {
    *(ctx as *const u32) as *mut u32
}
