// original: 0x00b8bf20 CREATE_MENU
/// Script native `CREATE_MENU` (hash 0x7DCA398F).
///
/// Forwards nine menu arguments to the engine: a handle, three float
/// bit-patterns, an integer, two booleans coerced with `arg != 0`, and two
/// more integers. No return slot is written.
/// Quirk (observed): each boolean is coerced into the low byte of a scratch
/// stack slot and pushed as a whole dword, so the pushed words' high bytes
/// are caller leftovers (entry ECX for the first, the context pointer for
/// the second). Only the low byte is behaviour; the rewrite pushes clean
/// 0/1 words and the contract compares only the low byte of those two call
/// arguments. The handler also clobbers its incoming stack slot, so the
/// stack channel is not compared.
export!(cdecl, rw_00b8bf20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), u32::from(*args.add(5) != 0), u32::from(*args.add(6) != 0), *args.add(7), *args.add(8))
    }
});
