// original: 0x0089EB70 aud_fwd_89E6E0
// ---------------------------------------------------------------------------
// 0x0089EB70: forward (this, arg) to the worker at 0x89E6E0.
// Same shape as 0x0089EB60. Pure forwarder.
// ---------------------------------------------------------------------------
export!(cdecl, rw_0089EB70(this_ptr: u32, arg: u32) -> u32 {
    callee_thiscall!(1, u32, this_ptr, arg)
});
