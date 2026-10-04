// original: 0x00bb2880 RESET_NO_LAW_VEHICLES_DESTROYED_BY_LOCAL_PLAYER
/// Script native `RESET_NO_LAW_VEHICLES_DESTROYED_BY_LOCAL_PLAYER` (hash 0x63615A6D).
///
/// Tail-jump thunk (observed): the handler body is a single jump to a shared implementation, forwarding the context pointer on the stack and returning its answer.
///
/// Transport note: the checker intercepts the jump by rewriting it into a call, so on the original side the stub observes the worker's return address one slot below the forwarded word. The rewrite passes a dummy word plus the context pointer and the contract skips the dummy slot, comparing only the forwarded word, the call identity and every other channel.
export!(cdecl, rw_00bb2880(ctx: *const u8) -> u32 {
    unsafe {
        // Slot 0 stands where the patched call's return address lands on
        // the original side (skipped by the contract); slot 1 is the real
        // forwarded argument, compared exactly.
        callee_cdecl!(1, u32, 0, ctx as u32)
    }
});
