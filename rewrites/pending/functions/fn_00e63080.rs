// original: 0x00e63080 cache_hash_frontend_game_tinny_right
/// Hash the audio preset name "FRONTEND_GAME_TINNY_RIGHT" through the engine's
/// string-hash helper, cache the hash in its global slot, and return it.
export!(cdecl, rw_00e63080() -> u32 {
    unsafe {
        let hash: u32 = callee_cdecl!(1, u32, relocated(0x00e82c68), 0);
        *global::<u32>(0x01176884) = hash;
        hash
    }
});
