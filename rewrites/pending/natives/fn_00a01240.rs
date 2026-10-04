// original: 0x00a01240 HAS_PICKUP_BEEN_COLLECTED
// Script native handler `HAS_PICKUP_BEEN_COLLECTED` (name: type) -> void.
//
// Public behaviour: Returns true if specified pickup has been collected [Pickup].
// Handler mechanics: calls one engine function, zero-extends the answer's low byte, and stores it in the return slot.
// The call context at `ctx` holds the return-slot pointer at +0 and the
// argument-array pointer at +8; argument slot `i` is the dword at `args + 4*i`.
checker_rt::export!(cdecl, rn30_00a01240(ctx: u32) -> () {
    const CTX_RET: u32 = 0; // offset of the return-slot pointer in the context
    const CTX_ARGS: u32 = 8; // offset of the argument-array pointer in the context
    let argv = unsafe { ((ctx + CTX_ARGS) as *const u32).read() };
    let slot = |i: u32| unsafe { ((argv + i * 4) as *const u32).read() };
    let answer: u32 = checker_rt::callee_cdecl!(1, u32, slot(0));
    // Only the low byte of the answer is kept (`movzx`), matching the original.
    let kept = answer & 0xff;
    let ret = unsafe { ((ctx + CTX_RET) as *const *mut u32).read() };
    unsafe { ret.write(kept) };
});
