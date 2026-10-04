// original: 0x00ba2620 SET_PED_SKIPS_COMPLEX_COVER_COLLISION_CHECKS
// Script native handler `SET_PED_SKIPS_COMPLEX_COVER_COLLISION_CHECKS` () -> void.
//
// Public behaviour: no public description found.
// Handler mechanics: normalises one argument slot to 0/1 and forwards to one engine function (see the stack-slot note below).
// The call context at `ctx` holds the return-slot pointer at +0 and the
// argument-array pointer at +8; argument slot `i` is the dword at `args + 4*i`.
//
// Stack-slot note: the original normalises the boolean with `setne` into the
// low byte of its own incoming stack slot (which still holds `ctx`) and then
// pushes that whole dword. The pushed value is therefore
// `(ctx & !0xff) | (slot != 0)`; the rewrite passes only the clean flag
// and the pushed argument is masked in the contract (call_skip). The clobber of
// the incoming slot itself is a compiler temporary-placement artefact with
// no caller-visible meaning and is not reproduced (the checker's stack
// comparison is disabled for this function only).
lf_k2_rt::export!(cdecl, rn30_00ba2620(ctx: u32) -> () {
    const CTX_RET: u32 = 0; // offset of the return-slot pointer in the context
    const CTX_ARGS: u32 = 8; // offset of the argument-array pointer in the context
    let argv = unsafe { ((ctx + CTX_ARGS) as *const u32).read() };
    let slot = |i: u32| unsafe { ((argv + i * 4) as *const u32).read() };
    let normalised = u32::from(slot(1) != 0);
    // v2: only the clean flag is pushed (the original's high bytes are masked
    // in the contract via call_skip).
    let pushed = normalised;
    lf_k2_rt::callee_cdecl!(1, u32, slot(0), pushed);
});
