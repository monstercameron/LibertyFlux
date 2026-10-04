// original: 0x00bd9870 WRITE_LOBBY_PREFERENCE
/// Native handler `WRITE_LOBBY_PREFERENCE`: write a lobby preference: forward the key and value words.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00bd9870(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1),);
        ans
    }
});
