// original: 0x00b9ef50 GET_CHAR_MODEL
// Script native handler `GET_CHAR_MODEL` (self: Char) -> model: int.
//
// Public behaviour:  [Char].
// Handler mechanics: reads its argument slots out of the call context and forwards them to one engine function, ignoring the answer.
// The call context at `ctx` holds the return-slot pointer at +0 and the
// argument-array pointer at +8; argument slot `i` is the dword at `args + 4*i`.
lf_k2_rt::export!(cdecl, rn30_00b9ef50(ctx: u32) -> () {
    const CTX_RET: u32 = 0; // offset of the return-slot pointer in the context
    const CTX_ARGS: u32 = 8; // offset of the argument-array pointer in the context
    let argv = unsafe { ((ctx + CTX_ARGS) as *const u32).read() };
    let slot = |i: u32| unsafe { ((argv + i * 4) as *const u32).read() };
    lf_k2_rt::callee_cdecl!(1, u32, slot(0), slot(1));
});
