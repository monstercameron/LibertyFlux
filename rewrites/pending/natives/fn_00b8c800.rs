// original: 0x00b8c800 GET_CORRECTED_COLOUR
/// Script native `GET_CORRECTED_COLOUR` (hash 0x64D35E1D).
///
/// Forwards six script arguments to the engine. No return slot is written.
export!(cdecl, rw_00b8c800(ctx: *const u8) -> u32 {
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
        )
    }
});
