// original: 0x00BD8110 NETWORK_ALL_PARTY_MEMBERS_PRESENT
/// F14 NETWORK_ALL_PARTY_MEMBERS_PRESENT: no-arg call, low byte of answer.
export!(cdecl, rn10_network_all_party_members_present(ctx: *const u32) -> u32 {
    unsafe {
        let (ret, _) = ctx_parts(ctx);
        let answer: u32 = callee_cdecl!(2, u32,);
        *ret = answer & 0xFF;
        ret as u32
    }
});
