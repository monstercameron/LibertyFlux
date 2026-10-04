// original: 0x00b8db60 TURN_OFF_RADIOHUD_IN_LOBBY
/// Script native `TURN_OFF_RADIOHUD_IN_LOBBY` (hash 0x4ED6764C).
///
/// Tail-jump thunk (observed): the handler body is a single jump to a shared implementation, forwarding the context pointer on the stack and returning its answer.
///
/// Transport note: the checker intercepts the jump by rewriting it into a call, so on the original side the stub observes the worker's return address one slot below the forwarded word. The rewrite passes a dummy word plus the context pointer and the contract skips the dummy slot, comparing only the forwarded word, the call identity and every other channel.
export!(cdecl, rw_00b8db60(ctx: *const u8) -> u32 {
    unsafe {
        // Slot 0 stands where the patched call's return address lands on
        // the original side (skipped by the contract); slot 1 is the real
        // forwarded argument, compared exactly.
        callee_cdecl!(1, u32, 0, ctx as u32)
    }
});
