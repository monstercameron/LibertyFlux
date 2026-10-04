// original: 0x00bd9750 STORE_DAMAGE_TRACKER_FOR_NETWORK_PLAYER
/// Native handler `STORE_DAMAGE_TRACKER_FOR_NETWORK_PLAYER`: store a damage tracker for a network player: forward the three words, return the engine result.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00bd9750(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2),);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = ans;
        ans
    }
});
