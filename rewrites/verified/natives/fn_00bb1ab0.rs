// original: 0x00bb1ab0 ALLOW_LOCKON_TO_RANDOM_PEDS
/// Script native `ALLOW_LOCKON_TO_RANDOM_PEDS` (hash 0x6FE455D8).
///
/// Forwards 2 script arguments (a player index and a boolean flag) to the engine.
///
/// No return slot is written.
///
/// Quirk (observed): the handler coerces the boolean argument into
/// the low byte of its own incoming stack slot and pushes the whole
/// dword, so the pushed word's high bytes repeat the context pointer.
/// The engine reads only the low byte (Inferred); the full dword is
/// reproduced here for bit-exact outgoing-call matching.
/// No return slot is written.
///
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00bb1ab0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        // v2 port: push the bare flag; the slot-residue high bytes are masked in the contract (call_skip).
        let quirked = flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
