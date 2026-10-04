// original: 0x00bb2630 IS_PLAYER_VEHICLE_ENTRY_DISABLED
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `IS_PLAYER_VEHICLE_ENTRY_DISABLED`.
///
/// Reports whether vehicle entry is disabled for the player.
///
/// Handler mechanics: takes the native call context,
/// Forwards the player index, then stores the low byte of the engine
/// answer in the return slot.
export!(cdecl, rw_00bb2630(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let ret = unsafe { *(ctx as *const u32) } as *mut u32;
    let player = unsafe { *args };
    let disabled = callee_cdecl!(1, u32, player) & 0xFF;
    unsafe { *ret = disabled };
});
