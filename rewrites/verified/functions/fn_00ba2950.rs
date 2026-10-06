// original: 0x00ba2950 SWITCH_PED_TO_RAGDOLL
/// Script native `SWITCH_PED_TO_RAGDOLL`.
///
/// Forwards 7 script arguments to the engine: argument 0 forwarded unchanged; argument 1 forwarded unchanged; argument 2 forwarded unchanged; argument 3 forwarded unchanged; argument 4 coerced to 0/1 in the low byte of a frame temporary (upper bytes unspecified, compared low byte only); argument 5 coerced to 0/1 in the low byte of a frame temporary (upper bytes unspecified, compared low byte only); argument 6 coerced to 0/1 in the low byte of a word whose upper bytes repeat the context pointer.
///
/// Stores the low byte of the engine answer (zero-extended) into the return slot and returns the slot pointer.
export!(cdecl, rw_00ba2950(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), u32::from(*args.add(4) != 0), u32::from(*args.add(5) != 0), (ctx as u32 & 0xFFFF_FF00) | u32::from(*args.add(6) != 0));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
