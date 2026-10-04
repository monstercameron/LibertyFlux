// original: 0x00ba06b0 LOCATE_CHAR_IN_CAR_CAR_2D
/// Script native `LOCATE_CHAR_IN_CAR_CAR_2D` (hash 0x53B429F9).
///
/// Returns true if the character is within the 2D radius of the vehicle in a vehicle
///
/// Script arguments: self:Char: ?, car:Car: ?, x:float: ?, y:float: ?, flag:bool: ?.
///
/// Forwards arg0 (integer/handle), arg1 (integer/handle), arg2 (float bit-pattern), arg3 (float bit-pattern), arg4 (boolean flag) to the engine routine.
/// Each flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces a flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// The engine's byte answer is zero-extended into the return slot.
export!(cdecl, rw_00ba06b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const *mut u32);
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        let arg2 = *args.add(2);
        let arg3 = *args.add(3);
        let flag4 = u32::from(*args.add(4) != 0);
        let arg4 = (ctx as u32 & 0xFFFF_FF00) | flag4;
        let answer: u32 = callee_cdecl!(1, u32, arg0, arg1, arg2, arg3, arg4, );
        *slot = answer & 0xFF;
        slot as u32
    }
});
