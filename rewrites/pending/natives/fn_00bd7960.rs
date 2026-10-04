// original: 0x00BD7960 GET_DESTROYER_OF_NETWORK_ID
/// F05 GET_DESTROYER_OF_NETWORK_ID: forwards 2 args, stores full answer.
export!(cdecl, rn10_get_destroyer_of_network_id(ctx: *const u32) -> u32 {
    unsafe {
        let (ret, args) = ctx_parts(ctx);
        let answer: u32 = callee_cdecl!(2, u32, *args, *args.add(1));
        *ret = answer;
        answer
    }
});
