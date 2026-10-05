// original: 0x00883240 stream_obj_op_thunk (proposed)
/// Forward two arguments to an inner object's slot-3 handler, or report -1.
///
/// Dereferences the handle: a null inner pointer returns `0xffffffff`
/// (`-1`). Otherwise tail-jumps to the inner object's slot-3 handler
/// (table at `+0x0c`; intercepted callee 1, thiscall: inner object in `ecx`,
/// the two arguments on the stack) and returns its result. The rewrite
/// performs the same load-and-call through the fabricated object, landing on
/// the same planted stub; the callee cleans the stack on both sides, so the
/// stack adjustment matches on the null path (the callee pops 8 bytes) and the forward path
/// alike.
///
/// Original: thiscall, two stack arguments, returns in `eax`.
lf_checker_rt::export!(thiscall, rw_00883240(handle: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const HANDLER_SLOT: u32 = 0x0c;
        const NONE: u32 = 0xFFFF_FFFF;
        let inner = (handle as *const u32).read_unaligned();
        if inner == 0 {
            NONE
        } else {
            let table = (inner as *const u32).read_unaligned();
            let handler = ((table + HANDLER_SLOT) as *const u32).read_unaligned();
            let forward: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(handler as usize);
            forward(inner, a1, a2)
        }
    }
});
