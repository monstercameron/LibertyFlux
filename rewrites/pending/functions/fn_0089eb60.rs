// original: 0x0089EB60 aud_fwd_89E500
// ---------------------------------------------------------------------------
// 0x0089EB60: forward (this, arg) to the worker at 0x89E500.
// Shape: push arg; ecx = this; call; ret. Callee takes ECX plus one stack
// word and cleans 4. Pure forwarder.
// ---------------------------------------------------------------------------
export!(cdecl, rw_0089EB60(this_ptr: u32, arg: u32) -> u32 {
    callee_thiscall!(1, u32, this_ptr, arg)
});
