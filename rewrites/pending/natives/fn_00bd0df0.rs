// original: 0x00bd0df0 GET_AMMO_IN_CLIP
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `GET_AMMO_IN_CLIP`.
///
/// Reads how much ammunition is in the clip of the character's current weapon.
///
/// Handler mechanics: takes the native call context,
/// Forwards character, weapon and destination slot, then stores the low
/// byte of the engine answer (clip ammo, or 0) in the return slot.
export!(cdecl, rw_00bd0df0(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let ret = unsafe { *(ctx as *const u32) } as *mut u32;
    let a0 = unsafe { *args };
    let a1 = unsafe { *args.add(1) };
    let a2 = unsafe { *args.add(2) };
    let ammo = callee_cdecl!(1, u32, a0, a1, a2) & 0xFF;
    unsafe { *ret = ammo };
});
