// original: 0x009e12f0 audio_list_gated_remove
/// When the gate accepts the argument, remove it from the list.
/// Returns the last helper answer. (cdecl/1)
export!(cdecl, rw_009e12f0(a: u32) -> u32 {
    let gate = callee_thiscall!(1, u32, relocated(0x12BD174), a);
    if gate & 0xFF != 0 {
        callee_thiscall!(2, u32, relocated(0x12BD174), a)
    } else {
        gate
    }
});
