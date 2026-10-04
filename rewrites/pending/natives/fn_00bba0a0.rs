// original: 0x00bba0a0 TASK_PERFORM_SEQUENCE
// Script native handler `TASK_PERFORM_SEQUENCE` (name: type, name: type) -> void.
//
// Public behaviour:  [Task].
// Handler mechanics: reads its argument slots out of the call context and forwards them to one engine function, ignoring the answer.
// The call context at `ctx` holds the return-slot pointer at +0 and the
// argument-array pointer at +8; argument slot `i` is the dword at `args + 4*i`.
lf_k2_rt::export!(cdecl, rn30_00bba0a0(ctx: u32) -> () {
    const CTX_RET: u32 = 0; // offset of the return-slot pointer in the context
    const CTX_ARGS: u32 = 8; // offset of the argument-array pointer in the context
    let argv = unsafe { ((ctx + CTX_ARGS) as *const u32).read() };
    let slot = |i: u32| unsafe { ((argv + i * 4) as *const u32).read() };
    lf_k2_rt::callee_cdecl!(1, u32, slot(0), slot(1));
});
