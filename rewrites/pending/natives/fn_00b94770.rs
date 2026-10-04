// original: 0x00b94770 GET_STRING_WIDTH_WITH_STRING
// Script native handler `GET_STRING_WIDTH_WITH_STRING` () -> void.
//
// Public behaviour: no public description found.
// Handler mechanics: calls one engine function returning a float and stores it in the return slot.
// The call context at `ctx` holds the return-slot pointer at +0 and the
// argument-array pointer at +8; argument slot `i` is the dword at `args + 4*i`.
lf_k2_rt::export!(cdecl, rn30_00b94770(ctx: u32) -> () {
    const CTX_RET: u32 = 0; // offset of the return-slot pointer in the context
    const CTX_ARGS: u32 = 8; // offset of the argument-array pointer in the context
    let argv = unsafe { ((ctx + CTX_ARGS) as *const u32).read() };
    let slot = |i: u32| unsafe { ((argv + i * 4) as *const u32).read() };
    // The engine function returns a 32-bit float on x87 ST0. The original
    // spills it over its own incoming argument slot (that address still
    // holds `ctx`) and reloads it into the
    // return slot. Copying the value stores bit-identical bytes for every
    // input including NaNs; the clobber of the incoming slot itself is
    // the same compiler scratch-slot artefact as in the bool handlers
    // (unwritable from safe Rust) and is excluded from the stack check.
    let answer: f32 = lf_k2_rt::callee_cdecl!(1, f32, slot(0), slot(1));
    let ret = unsafe { ((ctx + CTX_RET) as *const *mut f32).read() };
    unsafe { ret.write(answer) };
});
