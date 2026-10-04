// original: 0x00b9e700 CREATE_CHAR_AS_PASSENGER
/// Script native `CREATE_CHAR_AS_PASSENGER` (hash 0x442B1C1D).
///
/// Forwards five script arguments (vehicle handle, model hash, seat index and
/// two further integers) to the engine. No return slot is written.
export!(cdecl, rw_00b9e700(ctx: *const u8) -> u32 {
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
        )
    }
});
