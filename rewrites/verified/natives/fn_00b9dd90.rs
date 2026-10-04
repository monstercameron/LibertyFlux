// original: 0x00b9dd90 ADD_ADDITIONAL_POPULATION_MODEL
/// Script native `ADD_ADDITIONAL_POPULATION_MODEL` (hash 0x7EDE120F).
///
/// Forwards 1 script argument(s) to the engine: 1 integer(s).
/// No return slot is written.
export!(cdecl, rw_00b9dd90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
