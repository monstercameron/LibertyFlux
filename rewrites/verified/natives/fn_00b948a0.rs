// original: 0x00b948a0 IS_BULLET_IN_BOX
/// Script native `IS_BULLET_IN_BOX` (hash 0x60964DB8).
///
/// Forwards seven script arguments (six box-bound coordinates and a boolean flag) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
///
/// The coordinates are floats, forwarded as raw bits. The flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its own incoming stack slot and pushes the whole dword, so the pushed word's high bytes repeat the context pointer. The engine reads only the low byte (Inferred); the full dword is reproduced here for bit-exact outgoing-call matching.
export!(cdecl, rw_00b948a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(6) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), quirked);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
