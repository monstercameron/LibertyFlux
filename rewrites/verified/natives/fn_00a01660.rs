// original: 0x00a01660 LOCATE_OBJECT_2D
/// Script native `LOCATE_OBJECT_2D` (hash 0x59A57BA8).
///
/// Forwards six script arguments (an object handle, four rectangle bounds and a boolean flag) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
///
/// The bounds are floats, forwarded as raw bits. The flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its own incoming stack slot and pushes the whole dword, so the pushed word's high bytes repeat the context pointer. The engine reads only the low byte (Inferred); the full dword is reproduced here for bit-exact outgoing-call matching.
export!(cdecl, rw_00a01660(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(5) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), quirked);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
