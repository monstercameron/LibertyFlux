// original: 0x00bd4610 UPDATE_PTFX_OFFSETS
/// Script native `UPDATE_PTFX_OFFSETS` (hash 0x45472E9D).
///
/// Forwards seven script arguments (a handle and six float bit-patterns). The call passes thirteen words: the seven arguments, a repeat of the second vector, then a repeat of the first (both by-value vectors are assembled in scratch that sits inside the caller's cleanup range, so all thirteen words are observed call arguments) to the engine. No return slot is written.
export!(cdecl, rw_00bd4610(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            *args.add(6),
            *args.add(4),
            *args.add(5),
            *args.add(6),
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
