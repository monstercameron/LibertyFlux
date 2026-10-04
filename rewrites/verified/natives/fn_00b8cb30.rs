// original: 0x00b8cb30 GET_STRING_FROM_STRING
// Script native handler `GET_STRING_FROM_STRING` () -> void.
//
// Public behaviour: no public description found.
// Handler mechanics: calls one engine function and stores the full 32-bit answer in the return slot.
// The call context at `ctx` holds the return-slot pointer at +0 and the
// argument-array pointer at +8; argument slot `i` is the dword at `args + 4*i`.
lf_k2_rt::export!(cdecl, rn30_00b8cb30(ctx: u32) -> () {
    const CTX_RET: u32 = 0; // offset of the return-slot pointer in the context
    const CTX_ARGS: u32 = 8; // offset of the argument-array pointer in the context
    let argv = unsafe { ((ctx + CTX_ARGS) as *const u32).read() };
    let slot = |i: u32| unsafe { ((argv + i * 4) as *const u32).read() };
    let answer: u32 = lf_k2_rt::callee_cdecl!(1, u32, slot(0), slot(1), slot(2));
    let kept = answer;
    let ret = unsafe { ((ctx + CTX_RET) as *const *mut u32).read() };
    unsafe { ret.write(kept) };
});
