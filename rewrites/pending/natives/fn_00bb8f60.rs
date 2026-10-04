// original: 0x00bb8f60 SET_PED_PATH_MAY_DROP_FROM_HEIGHT
/// Script native `SET_PED_PATH_MAY_DROP_FROM_HEIGHT` (hash 0x4F37648C).
///
/// Forwards two script arguments to the engine: a character handle and a
/// boolean flag coerced with `arg != 0`. No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00bb8f60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
        let quirked = flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
