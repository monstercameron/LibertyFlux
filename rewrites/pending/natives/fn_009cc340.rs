// original: 0x009cc340 REPORT_POLICE_SPOTTING_SUSPECT
/// Script native handler `REPORT_POLICE_SPOTTING_SUSPECT`.
///
/// Reports a police spotting of the suspect.
///
/// Forwards 1 script argument to the engine function; no return slot.
/// handler function: `0x009cc340`, engine call site: `0x009cc349`.
export!(cdecl, rw_9cc340(ctx: *const NativeCtx) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let a0 = *args.add(0);
        callee_cdecl!(1, u32, a0)
    }
});
