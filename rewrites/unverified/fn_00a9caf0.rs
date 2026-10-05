// original: 0x00a9caf0 stream_node_resolve_and_emit (proposed)

/// Resolve a node's payload pointer and emit its two header words.
///
/// `this` points to a node with a state word at `+0x70`. A zero state
/// returns the incoming `eax` untouched, which a Rust rewrite cannot
/// observe, so the contract pins the state non-zero and that path never
/// runs (see the `narrowed` note in `results.json`). Otherwise the target
/// object at `+0x68` is asked (virtual slot `+0xa0`, object in ECX) for the
/// payload; a null answer falls back to the word at target `+0x38`. The
/// emitter is then called with `[[payload + 4] + 0xc]` in ECX and stack
/// arguments (`[this + 8]`, `[this + 0x70]`), and its answer is the result.
///
/// Original: 0x00a9caf0 (thiscall, no stack arguments; one virtual call).
lf_checker_rt::export!(thiscall, rw_00a9caf0(this: u32) -> u32 {
    unsafe {
        const STATE_OFF: u32 = 0x70;
        const TARGET_OFF: u32 = 0x68;
        const HEAD_OFF: u32 = 8;
        const VTABLE_SLOT: u32 = 0xa0;
        const FALLBACK_OFF: u32 = 0x38;
        const EMIT: u32 = 2;
        let state = ((this + STATE_OFF) as *const u32).read_unaligned();
        if state == 0 {
            // Dead in the contract (state pinned non-zero): the original
            // returns entry eax here. A defined wrong value, never a panic
            // (a panicking rewrite wedges the worker instead of failing).
            return 0xdead_beef;
        }
        let target = ((this + TARGET_OFF) as *const u32).read_unaligned();
        let vtable = (target as *const u32).read_unaligned();
        let slot = ((vtable + VTABLE_SLOT) as *const u32).read_unaligned();
        let ask: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        let got: u32 = ask(target);
        let payload = if got == 0 {
            ((target + FALLBACK_OFF) as *const u32).read_unaligned()
        } else {
            got
        };
        let mid = ((payload + 4) as *const u32).read_unaligned();
        let emit_this = ((mid + 0x0c) as *const u32).read_unaligned();
        let head = ((this + HEAD_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(EMIT, u32, emit_this, head, state)
    }
});
