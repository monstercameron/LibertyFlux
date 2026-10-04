// original: 0x009cc790 SKIP_TO_NEXT_SCRIPTED_CONVERSATION_LINE
/// Script native handler `SKIP_TO_NEXT_SCRIPTED_CONVERSATION_LINE` (hash 0x294C35B0).
///
/// The original body is a single jump to a shared implementation at 0x009CDD00;
/// the rewrite forwards the call context and returns the answer.
export!(cdecl, rw_009cc790(ctx: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, ctx) }
});
