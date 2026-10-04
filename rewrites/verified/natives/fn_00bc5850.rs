// original: 0x00bc5850 FORCE_ALL_VEHICLE_LIGHTS_OFF
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `FORCE_ALL_VEHICLE_LIGHTS_OFF`.
///
/// Disables all vehicle lights from being rendered when enabled.
///
/// Handler mechanics: takes the native call context,
/// Forwards the coerced state flag to the engine.
/// The pushed flag dword's high bytes are dead-slot residue (see body);
/// the contract masks that call argument.
export!(cdecl, rw_00bc5850(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let flag = u32::from(unsafe { *args } != 0);
    // Quirk: the original coerces the boolean in place in its own argument
    // slot and forwards the whole slot dword. Only the 0/1 flag is passed
    // here; the residue high bytes are masked in the contract (call_skip).
    let coerced = flag;
    callee_cdecl!(1, u32, coerced);
});
