// original: 0x00b8d680 SET_MENU_COLUMN
// Script native handler `SET_MENU_COLUMN` (name: type, name: type, name: type, name: type, name: type, name: type, name: type, name: type, name: type, name: type, name: type, name: type, name: type, name: type, name: type) -> void.
//
// Public behaviour:  [Menu].
// Handler mechanics: reads its argument slots out of the call context and forwards them to one engine function, ignoring the answer.
// The call context at `ctx` holds the return-slot pointer at +0 and the
// argument-array pointer at +8; argument slot `i` is the dword at `args + 4*i`.
checker_rt::export!(cdecl, rn30_00b8d680(ctx: u32) -> () {
    const CTX_RET: u32 = 0; // offset of the return-slot pointer in the context
    const CTX_ARGS: u32 = 8; // offset of the argument-array pointer in the context
    let argv = unsafe { ((ctx + CTX_ARGS) as *const u32).read() };
    let slot = |i: u32| unsafe { ((argv + i * 4) as *const u32).read() };
    checker_rt::callee_cdecl!(1, u32, slot(0), slot(1), slot(2), slot(3), slot(4), slot(5), slot(6), slot(7), slot(8), slot(9), slot(10), slot(11), slot(12), slot(13), slot(14));
});
