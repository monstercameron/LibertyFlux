// original: 0x00ba1680 SET_CHAR_MAX_MOVE_BLEND_RATIO
// Script native handler `SET_CHAR_MAX_MOVE_BLEND_RATIO` (name: type, name: type) -> void.
//
// Public behaviour:  [Char].
// Handler mechanics: forwards its slots to one engine function; one slot carries float bits passed through unchanged.
// The call context at `ctx` holds the return-slot pointer at +0 and the
// argument-array pointer at +8; argument slot `i` is the dword at `args + 4*i`.
checker_rt::export!(cdecl, rn30_00ba1680(ctx: u32) -> () {
    const CTX_RET: u32 = 0; // offset of the return-slot pointer in the context
    const CTX_ARGS: u32 = 8; // offset of the argument-array pointer in the context
    let argv = unsafe { ((ctx + CTX_ARGS) as *const u32).read() };
    let slot = |i: u32| unsafe { ((argv + i * 4) as *const u32).read() };
    // Slot(s) [1] carry float bits; the original shuttles them through an
    // SSE register onto the stack without conversion, so passing the raw
    // bits pushes a bit-identical dword.
    checker_rt::callee_cdecl!(1, u32, slot(0), slot(1));
});
