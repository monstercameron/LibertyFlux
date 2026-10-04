// original: 0x009cc180 PLAY_SOUND_FROM_VEHICLE
/// Play a named sound attached to a vehicle.
///
/// Forwards the sound id, sound name and vehicle handle (script arguments
/// 0-2) to the engine implementation. Returns whatever it returned.
export!(cdecl, rw_009cc180(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let (sound_id, name, vehicle) = (*args, *args.add(1), *args.add(2));
        callee_cdecl!(1, u32, sound_id, name, vehicle)
    }
});
