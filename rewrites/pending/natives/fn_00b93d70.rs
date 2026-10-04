// original: 0x00b93d70 ADD_POLICE_RESTART
/// Script native `ADD_POLICE_RESTART` (hash 0x42492860).
///
/// Forwards five script arguments to the engine: four float bit-patterns
/// (position and heading) and one integer. The floats are only copied onto
/// the stack, so they are forwarded as raw bits. No return slot is written.
export!(cdecl, rw_00b93d70(ctx: *const u8) -> u32 {
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
