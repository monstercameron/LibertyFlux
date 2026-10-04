// original: 0x00bd7b30 GET_NETWORK_TIMER
/// Native handler `GET_NETWORK_TIMER`: read the network timer: forward the player slot (result goes through the pointer).
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00bd7b30(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0),);
        ans
    }
});
