// original: 0x00ba1700 SET_CHAR_MELEE_ACTION_FLAG1
// Script native handler `SET_CHAR_MELEE_ACTION_FLAG1` (name: type, name: type) -> void.
//
// Public behaviour:  [Char].
// Handler mechanics: normalises one argument slot to 0/1 and forwards to one engine function (see the stack-slot note below).
// The call context at `ctx` holds the return-slot pointer at +0 and the
// argument-array pointer at +8; argument slot `i` is the dword at `args + 4*i`.
//
// Stack-slot note: the original normalises the boolean with `setne` into the
// low byte of its own incoming stack slot (which still holds `ctx`) and then
// pushes that whole dword. The pushed value is therefore
// `(ctx & !0xff) | (slot != 0)`, reproduced exactly below; the clobber of
// the incoming slot itself is a compiler temporary-placement artefact with
// no caller-visible meaning and is not reproduced (the checker's stack
// comparison is disabled for this function only).
checker_rt::export!(cdecl, rn30_00ba1700(ctx: u32) -> () {
    const CTX_RET: u32 = 0; // offset of the return-slot pointer in the context
    const CTX_ARGS: u32 = 8; // offset of the argument-array pointer in the context
    let argv = unsafe { ((ctx + CTX_ARGS) as *const u32).read() };
    let slot = |i: u32| unsafe { ((argv + i * 4) as *const u32).read() };
    let normalised = u32::from(slot(1) != 0);
    // See the stack-slot note above: the pushed dword keeps the incoming
    // `ctx` value in its high three bytes.
    let pushed = (ctx & 0xffff_ff00) | normalised;
    checker_rt::callee_cdecl!(1, u32, slot(0), pushed);
});
