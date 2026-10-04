// original: 0x00968540 init_timer_groups
/// Initialise the twelve timer blocks in the four groups at `this`.
///
/// Calls the block initialiser (0x9684C0, thiscall, no stack arguments)
/// twelve times, in order, with the block addresses `this + 0x10`,
/// `+0x70`, `+0xD0`, `+0x130`, `+0x190`, `+0x1F0`, `+0x250`, `+0x2B0`,
/// `+0x310`, `+0x370`, `+0x3D0`, `+0x430`. No return value.
///
/// Original: 0x00968540 (thiscall, no stack words).

export!(thiscall, rw_00968540(this: u32) -> u32 {
    for off in [
        0x10u32, 0x70, 0xD0, 0x130, 0x190, 0x1F0, 0x250, 0x2B0, 0x310, 0x370, 0x3D0, 0x430,
    ] {
        callee_thiscall!(1, u32, this.wrapping_add(off));
    }
    0
});
