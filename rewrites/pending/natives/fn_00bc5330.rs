// original: 0x00bc5330 CLOSE_GARAGE
// Script native handler `CLOSE_GARAGE` (garageName: string) -> void.
//
// Public behaviour: Closes the garage [Garage].
// Handler mechanics: reads its argument slots out of the call context and forwards them to one engine function, ignoring the answer.
// The call context at `ctx` holds the return-slot pointer at +0 and the
// argument-array pointer at +8; argument slot `i` is the dword at `args + 4*i`.
checker_rt::export!(cdecl, rn30_00bc5330(ctx: u32) -> () {
    const CTX_RET: u32 = 0; // offset of the return-slot pointer in the context
    const CTX_ARGS: u32 = 8; // offset of the argument-array pointer in the context
    let argv = unsafe { ((ctx + CTX_ARGS) as *const u32).read() };
    let slot = |i: u32| unsafe { ((argv + i * 4) as *const u32).read() };
    checker_rt::callee_cdecl!(1, u32, slot(0));
});
