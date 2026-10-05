// original: 0x008e0240 pool_context_create (proposed)

/// Allocate and install the pool context.
///
/// Allocates the 28-byte context (callee 1); when that fails the context
/// global is cleared and 0 returned. Otherwise the block is initialised
/// through callee 2 (called with the block and the two arguments plus the
/// constant 16), the answer is published into the context global and also
/// returned. Cdecl, two stack arguments.
lf_checker_rt::export!(cdecl, rw_008e0240(a0: u32, a1: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x0117_64c0;
        const CTX_SIZE: u32 = 0x1c;
        const INIT_TAG: u32 = 0x10;
        const CALLEE_ALLOC: u32 = 1;
        const CALLEE_INIT: u32 = 2;
        let block = lf_checker_rt::callee_cdecl!(CALLEE_ALLOC, u32, CTX_SIZE);
        if block == 0 {
            lf_checker_rt::global::<u32>(CTX).write_unaligned(0);
            return 0;
        }
        let ctx = lf_checker_rt::callee_thiscall!(CALLEE_INIT, u32, block, a0, a1, INIT_TAG);
        lf_checker_rt::global::<u32>(CTX).write_unaligned(ctx);
        ctx
    }
});
