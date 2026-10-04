// original: 0x00BD82B0 NETWORK_FIND_GAME
// NETWORK_FIND_GAME: forwards 4 script words to the engine. No return.
export!(cdecl, rw_fn_bd82b0(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let _ans: u32 = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3));
    }
});
