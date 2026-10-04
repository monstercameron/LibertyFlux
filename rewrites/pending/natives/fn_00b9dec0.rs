// original: 0x00b9dec0 ALLOW_AUTO_CONVERSATION_LOOKATS
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `ALLOW_AUTO_CONVERSATION_LOOKATS`.
///
/// Lets a character automatically look at whoever is talking in a conversation.
///
/// Handler mechanics: takes the native call context,
/// Forwards the character handle and the coerced allow flag to the engine.
/// The pushed flag dword's high bytes are dead-slot residue (see body);
/// the contract masks that call argument.
export!(cdecl, rw_00b9dec0(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let ped = unsafe { *args };
    let flag = u32::from(unsafe { *args.add(1) } != 0);
    // Quirk: the original coerces the boolean in place in its own argument
    // slot and forwards the whole slot dword. Only the 0/1 flag is passed
    // here; the residue high bytes are masked in the contract (call_skip).
    let coerced = flag;
    callee_cdecl!(1, u32, ped, coerced);
});
