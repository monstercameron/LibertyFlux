// original: 0x00b8c970 GET_LENGTH_OF_STRING_WITH_THIS_TEXT_LABEL_INS_NUM
/// Script native `GET_LENGTH_OF_STRING_WITH_THIS_TEXT_LABEL_INS_NUM` (hash 0x5F02084D).
///
/// Forwards 3 script argument(s) to the engine: 2 integer(s), a boolean flag.
/// The flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// Stores the full engine answer into the return slot.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00b8c970(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        // v2 port: push the bare flag; the slot-residue high bytes are masked in the contract (call_skip).
        let quirked = flag;
        let answer = callee_cdecl!(1, u32, *args, quirked, *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
