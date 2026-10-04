// original: 0x00bd4000 GET_PHYSICAL_SCREEN_RESOLUTION
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `GET_PHYSICAL_SCREEN_RESOLUTION`.
///
/// Reads the physical screen resolution through two out-pointers.
///
/// Handler mechanics: takes the native call context,
/// Forwards both script pointers to the engine, which writes width and
/// height through them; nothing is stored in the return slot.
export!(cdecl, rw_00bd4000(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let a0 = unsafe { *args };
    let a1 = unsafe { *args.add(1) };
    callee_cdecl!(1, u32, a0, a1);
});
