// original: 0x00bc6c70 IS_HELI_PART_BROKEN
/// Script native `IS_HELI_PART_BROKEN`.
///
/// Forwards 4 script arguments to the engine: argument 0 forwarded unchanged; argument 1 coerced to 0/1 in the low byte of a frame temporary (upper bytes unspecified, compared low byte only); argument 2 coerced to 0/1 in the low byte of a frame temporary (upper bytes unspecified, compared low byte only); argument 3 coerced to 0/1 in the low byte of a word whose upper bytes repeat the context pointer.
///
/// Stores the low byte of the engine answer (zero-extended) into the return slot and returns the slot pointer.
export!(cdecl, rw_00bc6c70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, u32::from(*args.add(1) != 0), u32::from(*args.add(2) != 0), (ctx as u32 & 0xFFFF_FF00) | u32::from(*args.add(3) != 0));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
