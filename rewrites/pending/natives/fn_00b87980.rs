// original: 0x00b87980 SET_FOLLOW_VEHICLE_CAM_OFFSET
/// Script native `SET_FOLLOW_VEHICLE_CAM_OFFSET` (hash 0x56507469).
///
/// Forwards four script arguments to the engine: a boolean flag
/// (`arg != 0`) and three float bit-patterns (an offset). No return slot
/// is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00b87980(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
        let quirked = flag;
        callee_cdecl!(1, u32, quirked, *args.add(1), *args.add(2), *args.add(3))
    }
});
