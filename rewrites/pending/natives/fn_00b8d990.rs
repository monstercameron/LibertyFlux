// original: 0x00b8d990 SET_TEXT_JUSTIFY
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `SET_TEXT_JUSTIFY`.
///
/// Sets whether drawn text is justified to fill an even rectangle.
///
/// Handler mechanics: takes the native call context,
/// Forwards the coerced flag to the engine.
/// The pushed flag dword's high bytes are dead-slot residue (see body);
/// the contract masks that call argument.
export!(cdecl, rw_00b8d990(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let flag = u32::from(unsafe { *args } != 0);
    // Quirk: the original coerces the boolean in place in its own argument
    // slot and forwards the whole slot dword. Only the 0/1 flag is passed
    // here; the residue high bytes are masked in the contract (call_skip).
    let coerced = flag;
    callee_cdecl!(1, u32, coerced);
});
