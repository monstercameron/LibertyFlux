// original: 0x00B9AAB0 GET_RANDOM_NETWORK_RESTART_NODE_OF_GROUP
// GET_RANDOM_NETWORK_RESTART_NODE_OF_GROUP: script native handler with a constant extra argument.
// Passes the call context plus one code address (a relocated
// function pointer the loader fixup adjusts) to one engine helper.
lf_rn04_rt::export!(cdecl, rw_00b9aab0(ctx: u32) -> u32 {
    // Code address the original pushes; it has a loader fixup entry,
    // so it must be relocated exactly like any global reference.
    let worker_fn: u32 = lf_rn04_rt::relocated(0x00B9CA00);
    lf_rn04_rt::callee_cdecl!(1, u32, worker_fn, ctx)
});
