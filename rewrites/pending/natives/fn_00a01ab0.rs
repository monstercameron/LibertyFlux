// original: 0x00a01ab0 SET_HEADING_OF_CLOSEST_OBJECT_OF_TYPE
/// Script native `SET_HEADING_OF_CLOSEST_OBJECT_OF_TYPE` (hash 0x7ABD4D4D).
///
/// Forwards seven script arguments to the engine: four float bit-patterns
/// (a position and a first heading), one integer (the object type), one
/// more float bit-pattern (the new heading), and a boolean flag coerced
/// with `arg != 0`. No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00a01ab0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(6) != 0);
        // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
        let quirked = flag;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            quirked,
        )
    }
});
