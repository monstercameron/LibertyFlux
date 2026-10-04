// original: 0x00d740a0 render_mode_is_active
// Mode-is-active check.
// Returns 1 when the mode id is one of {0, 1, 4, 5}, else 0.
// The original dispatches through an internal jump table; the mapping is
// expressed directly here. Exit EAX above AL is entry residue (ret: al).
export!(stdcall, rw_00d740a0(mode: u32) -> u32 {
    match mode {
        0 | 1 | 4 | 5 => 1,
        _ => 0,
    }
});
