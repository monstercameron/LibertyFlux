// original: 0x00bc79c0 SET_HAS_BEEN_OWNED_BY_PLAYER
/// Script native `SET_HAS_BEEN_OWNED_BY_PLAYER` (hash 0x25750E4F).
///
/// Marks a vehicle as having been owned by the player. Forwards the vehicle
/// handle and a boolean flag coerced with `arg != 0`.
///
/// Quirk (observed): as in `ALLOW_REACTION_ANIMS`, the flag is coerced into
/// the low byte of the handler's own incoming stack slot, so the pushed
/// dword's high bytes repeat the context pointer; reproduced here exactly.
/// No return slot is written.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00bc79c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        // v2 port: push the bare flag; the slot-residue high bytes are masked in the contract (call_skip).
        let quirked = flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
