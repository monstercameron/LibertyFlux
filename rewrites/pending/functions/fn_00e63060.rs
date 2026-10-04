// original: 0x00e63060 cache_hash_frontend_game_tinny_left
/// Hash the audio preset name "FRONTEND_GAME_TINNY_LEFT" through the engine's
/// string-hash helper, cache the hash in its global slot, and return it.
export!(cdecl, rw_00e63060() -> u32 {
    unsafe {
        let hash: u32 = callee_cdecl!(1, u32, relocated(0x00e82c48), 0);
        *global::<u32>(0x01176880) = hash;
        hash
    }
});
