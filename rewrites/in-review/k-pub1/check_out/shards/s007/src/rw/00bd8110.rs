// original: 0x00BD8110 NETWORK_ALL_PARTY_MEMBERS_PRESENT
/// F14 NETWORK_ALL_PARTY_MEMBERS_PRESENT: no-arg call, low byte of answer.
export!(cdecl, rn10_network_all_party_members_present(ctx: *const u32) -> u32 {
    unsafe {
        let (ret, _) = ctx_parts(ctx);
        let answer: u32 = __lf_callee_cdecl!(2, u32,);
        *ret = answer & 0xFF;
        ret as u32
    }
});

/// Read the return slot and argument pointers from a call context (r-n10).
#[inline(always)]
unsafe fn ctx_parts(ctx: *const u32) -> (*mut u32, *const u32) {
    ((*ctx.add(RET_OFF_WORDS)) as *mut u32, (*ctx.add(ARGS_OFF_WORDS)) as *const u32)
}

/// Byte offset of the argument-array pointer in the call context (words).
const ARGS_OFF_WORDS: usize = 2;

/// Byte offset of the return-slot pointer in the call context (words).
const RET_OFF_WORDS: usize = 0;
