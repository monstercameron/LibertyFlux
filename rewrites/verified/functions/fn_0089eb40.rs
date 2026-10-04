// original: 0x0089EB40 aud_fwd_89E410
// ---------------------------------------------------------------------------
// 0x0089EB40: forward (this, b, c) to the worker at 0x89E410.
// Shape: push c; ecx = this; push b; call; ret. Takes three stack words even
// though it looks like two at first glance: the first push reads [esp+0xC].
// Callee takes ECX plus two stack words and cleans 8. Pure forwarder.
// ---------------------------------------------------------------------------
export!(cdecl, rw_0089EB40(this_ptr: u32, b: u32, c: u32) -> u32 {
    callee_thiscall!(1, u32, this_ptr, b, c)
});
