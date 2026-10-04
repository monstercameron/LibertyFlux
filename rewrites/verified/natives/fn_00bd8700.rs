// original: 0x00bd8700 NETWORK_INVITE_FRIEND
/// Native handler `NETWORK_INVITE_FRIEND`: invite a friend to a network game: forward the two invite words, return 0/1.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00bd8700(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1),);
        // The engine returns a byte-wide boolean; the handler stores it
        // zero-extended to 32 bits.
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = ans & 0xFF;
        slot as u32
    }
});
