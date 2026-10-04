// original: 0x00e630a0 cache_hash_warehouse_room_tone
/// Hash the audio preset name "WAREHOUSE_ROOM_TONE" through the engine's
/// string-hash helper, cache the hash in its global slot, and return it.
export!(cdecl, rw_00e630a0() -> u32 {
    unsafe {
        let hash: u32 = callee_cdecl!(1, u32, relocated(0x00e82c84), 0);
        *global::<u32>(0x01176d44) = hash;
        hash
    }
});
