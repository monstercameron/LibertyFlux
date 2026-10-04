// original: 0x00bb8a60 DOES_SCENARIO_EXIST_IN_AREA
/// Script native `DOES_SCENARIO_EXIST_IN_AREA` (hash 0x48252E33).
///
/// Forwards four float words and one boolean flag to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot. The
/// floats travel bit-for-bit; only the flag is interpreted (`arg != 0`).
///
/// Quirk (observed): the flag is coerced into the low byte of the handler's
/// own incoming stack slot and pushed as a whole dword, so its high bytes
/// repeat the context pointer. Reproduced here for bit-exact matching.
export!(cdecl, rw_00bb8a60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(4) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            quirked
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
