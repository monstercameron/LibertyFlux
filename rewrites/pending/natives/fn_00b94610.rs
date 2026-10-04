// original: 0x00b94610 GET_MODEL_DIMENSIONS
// Script native handler `GET_MODEL_DIMENSIONS` (model: int) -> pMinVector: Vector3, pMaxVector: Vector3.
//
// Public behaviour: no public description found.
// Handler mechanics: passes the call context itself plus a fixed code address to one engine function.
// The call context at `ctx` holds the return-slot pointer at +0 and the
// argument-array pointer at +8; argument slot `i` is the dword at `args + 4*i`.
lf_k2_rt::export!(cdecl, rn30_00b94610(ctx: u32) -> () {
    const CTX_RET: u32 = 0; // offset of the return-slot pointer in the context
    const CTX_ARGS: u32 = 8; // offset of the argument-array pointer in the context
    // The fixed address is a code address baked into the original as an
    // absolute immediate (it has a relocation entry); derive it from the
    // relocated image base like any other original address.
    let fixed = lf_k2_rt::relocated(0x00b96490);
    // Push order in the original is ctx first, fixed address second, so the
    // callee receives (fixed, ctx): last pushed is the first argument.
    lf_k2_rt::callee_cdecl!(1, u32, fixed, ctx);
});
