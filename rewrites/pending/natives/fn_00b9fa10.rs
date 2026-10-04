// original: 0x00b9fa10 IS_CHAR_IN_AREA_2D
/// Script native `IS_CHAR_IN_AREA_2D` (hash 0x7F371477).
///
/// Forwards six script arguments to the engine area test: a character
/// handle, four coordinate words copied as raw bits, and a boolean flag
/// (`arg != 0`). Stores the low byte of the answer into the return slot.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The full dword is reproduced here
/// for bit-exact outgoing-call matching.
export!(cdecl, rw_00b9fa10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const u32) as *mut u32;
        let flag = u32::from(*args.add(5) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            quirked
        );
        *slot = answer & 0xFF;
        slot as u32
    }
});
