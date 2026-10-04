// original: 0x00b94970 IS_INTERIOR_SCENE
// Script native handler `IS_INTERIOR_SCENE` () -> void.
//
// Public behaviour: no public description found.
// Handler mechanics: calls one engine function, zero-extends the answer's low byte, and stores it in the return slot.
// The call context at `ctx` holds the return-slot pointer at +0 and the
// argument-array pointer at +8; argument slot `i` is the dword at `args + 4*i`.
checker_rt::export!(cdecl, rn30_00b94970(ctx: u32) -> () {
    const CTX_RET: u32 = 0; // offset of the return-slot pointer in the context
    const CTX_ARGS: u32 = 8; // offset of the argument-array pointer in the context
    let answer: u32 = checker_rt::callee_cdecl!(1, u32,  );
    // Only the low byte of the answer is kept (`movzx`), matching the original.
    let kept = answer & 0xff;
    let ret = unsafe { ((ctx + CTX_RET) as *const *mut u32).read() };
    unsafe { ret.write(kept) };
});
