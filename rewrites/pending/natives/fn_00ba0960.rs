// original: 0x00ba0960 LOCATE_CHAR_ON_FOOT_CAR_2D
/// Script native `LOCATE_CHAR_ON_FOOT_CAR_2D` (hash 0x78A75EF4).
///
/// Forwards five script arguments to the engine: a character handle, a vehicle handle, two float bit-patterns (the area half-extents) and a boolean flag; then stores the low byte of the engine answer (zero-extended) into the return slot. The flag uses the stack-slot quirk below (Quirk (observed): the handler coerces the flag into the low byte of its own incoming stack slot and pushes the whole dword, so the pushed word's high bytes repeat the context pointer. The engine reads only the low byte (Inferred); the full dword is reproduced here for bit-exact outgoing-call matching.), so its pushed word's high bytes repeat the context pointer.
export!(cdecl, rw_00ba0960(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        // args 2 and 3 are float bit-patterns, forwarded untouched.
        let flag = u32::from(*args.add(4) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), quirked);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
