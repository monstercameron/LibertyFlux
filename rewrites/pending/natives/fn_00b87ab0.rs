// original: 0x00b87ab0 SET_HINT_ADVANCED_PARAMS
/// Script native `SET_HINT_ADVANCED_PARAMS` (hash 0x2E096356).
///
/// Forwards five script arguments to the engine: four float bit-patterns
/// and a flag coerced to 0/1. No return slot is written.
///
/// Quirk (observed): the flag's low byte is set into the handler's own
/// incoming stack slot and the whole dword is pushed, so its high bytes
/// repeat the context pointer. Reproduced here for bit-exact
/// outgoing-call matching.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00b87ab0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(4) != 0);
        // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
        let quirked = flag;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            quirked,
        )
    }
});
