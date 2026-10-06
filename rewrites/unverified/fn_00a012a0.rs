// original: 0x00a012a0 IS_CLOSEST_OBJECT_OF_TYPE_SMASHED_OR_DAMAGED
/// Script native `IS_CLOSEST_OBJECT_OF_TYPE_SMASHED_OR_DAMAGED`.
///
/// Forwards 7 script arguments to the engine: argument 0 forwarded as raw bits; argument 1 forwarded as raw bits; argument 2 forwarded as raw bits; argument 3 forwarded as raw bits; argument 4 forwarded unchanged; argument 5 coerced to 0/1 in the low byte of a caller-register temporary (upper bytes unspecified, compared low byte only); argument 6 coerced to 0/1 in the low byte of a word whose upper bytes repeat the context pointer.
///
/// Stores the low byte of the engine answer (zero-extended) into the return slot and returns the slot pointer.
export!(cdecl, rw_00a012a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), u32::from(*args.add(5) != 0), (ctx as u32 & 0xFFFF_FF00) | u32::from(*args.add(6) != 0));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
