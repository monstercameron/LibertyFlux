// original: 0x006952e0 anim_register_statics
/// Register the thirteen static animation objects in order.
export!(cdecl, rs80_6952e0() -> u32 {
    const OBJECTS: [u32; 13] = [
        0x01110298, 0x01110250, 0x01110274, 0x01110280, 0x011102BC, 0x01110244, 0x0111025C,
        0x011102A4, 0x011102B0, 0x01110238, 0x01110268, 0x011102C8, 0x0111028C,
    ];
    let mut r: u32 = 0;
    for o in OBJECTS {
        r = callee_thiscall!(1, u32, relocated(o));
    }
    r
});
