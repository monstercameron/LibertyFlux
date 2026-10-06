// original: 0x00b062e0 reset_state_block
/// Reset a state block to its defaults.
///
/// thiscall `(this)`: runs two init helpers (thiscall, no stack args),
/// then writes a fixed set of zero, -1 and float-constant words across
/// the block (`+0x160`..`+0x1ec`). Returns the second helper's answer
/// (the original leaves it in the return register).
export!(thiscall, rw_00b062e0(this: u32) -> u32 {
    const THIRTY: u32 = 0x41F0_0000; // 30.0f
    const SMALL: u32 = 0x3C75_C28F; // ~0.0138f
    const NEG_ONE: u32 = 0xBF80_0000; // -1.0f
    const POINT_NINE: u32 = 0x3F66_6666; // ~0.9f
    let _: u32 = callee_thiscall!(1, u32, this);
    let r: u32 = callee_thiscall!(2, u32, this);
    unsafe {
        let w = |off: u32, v: u32| ((this + off) as *mut u32).write_unaligned(v);
        w(0x178, 0);
        w(0x174, 0);
        w(0x170, 0);
        w(0x180, 0xFFFF_FFFF);
        w(0x184, 0xFFFF_FFFF);
        w(0x168, 0);
        w(0x164, 0);
        w(0x160, 0);
        w(0x188, 0);
        w(0x18C, 0);
        w(0x190, 0);
        w(0x19C, THIRTY);
        w(0x1A8, SMALL);
        w(0x1AC, SMALL);
        w(0x1B0, 0);
        w(0x1B4, 0);
        w(0x1A0, 0);
        w(0x1B8, NEG_ONE);
        w(0x1C0, POINT_NINE);
        ((this + 0x1EC) as *mut u8).write(0);
    }
    r
});
