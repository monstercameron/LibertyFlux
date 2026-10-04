// original: 0x00bc58d0 FORCE_RANDOM_CAR_MODEL
/// Script native `FORCE_RANDOM_CAR_MODEL` (hash 0x521D0D5B).
///
/// Forwards one script argument (a vehicle model id) to the engine, which
/// forces subsequently spawned random vehicles to use it. No return slot
/// is written.
export!(cdecl, rw_00bc58d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
