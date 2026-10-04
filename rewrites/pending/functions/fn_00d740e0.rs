// original: 0x00d740e0 render_mode_is_shadow
// Mode-is-shadow check.
// Returns 1 when the mode id is one of {8, 9, 10}, else 0.
// Exit EAX above AL is entry residue (ret: al).
export!(stdcall, rw_00d740e0(mode: u32) -> u32 {
    match mode {
        8 | 9 | 10 => 1,
        _ => 0,
    }
});
