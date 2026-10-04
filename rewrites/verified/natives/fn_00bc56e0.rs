// original: 0x00bc56e0 EXPLODE_CAR
/// Script native `EXPLODE_CAR` (hash 0x505518A2).
///
/// Forwards a vehicle handle and two booleans (coerced with `arg != 0`)
/// to the engine, which explodes the vehicle. No return slot is written.
/// Quirk (observed): each boolean is coerced into the low byte of a scratch
/// stack slot and pushed as a whole dword, so the pushed words' high bytes
/// are caller leftovers (entry ECX for the first, the context pointer for
/// the second). Only the low byte is behaviour; the rewrite pushes clean
/// 0/1 words and the contract compares only the low byte of those two call
/// arguments. The handler also clobbers its incoming stack slot, so the
/// stack channel is not compared.
export!(cdecl, rw_00bc56e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, u32::from(*args.add(1) != 0), u32::from(*args.add(2) != 0))
    }
});
