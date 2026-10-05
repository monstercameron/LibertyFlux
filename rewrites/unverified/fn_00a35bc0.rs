// original: 0x00a35bc0 vehicle_trio_init (proposed)

/// Initialise a vehicle object's three fixed regions and two sub-objects.
///
/// Runs the sub-object initialiser (id 1, thiscall/0) on `obj + 0x10`,
/// stamps three index half-words (`+0xe0`, `+0x110`, `+0x140`) with
/// `0xFFFF`, writes the constant header (`+0` zero, `+0x4`/`+0x8` 1000.0,
/// `+0xc` zero),
/// zeroes six words at `+0x1a0`..`+0x1b8` and the float block at
/// `+0xc0`..`+0xd4`, then runs the limits initialiser (id 2, thiscall/0)
/// on `obj`. Thiscall, no stack arguments, returns `obj`.
lf_checker_rt::export!(thiscall, rw_00a35bc0(obj: u32) -> u32 {
    unsafe {
        const SUB_INIT: u32 = 1;
        const LIMITS_INIT: u32 = 2;
        const NO_INDEX: u16 = 0xFFFF;
        const RATE: u32 = 0x447A_0000; // 1000.0
        let _: u32 = lf_checker_rt::callee_thiscall!(SUB_INIT, u32, obj.wrapping_add(0x10));
        // Half-word stores: the upper bytes of each word keep their fill.
        for off in [0xE0u32, 0x110, 0x140] {
            core::ptr::write_unaligned((obj + off) as *mut u16, NO_INDEX);
        }
        core::ptr::write_unaligned(obj as *mut u32, 0);
        core::ptr::write_unaligned((obj + 4) as *mut u32, RATE);
        core::ptr::write_unaligned((obj + 8) as *mut u32, RATE);
        core::ptr::write_unaligned((obj + 0xC) as *mut u32, 0);
        for off in [0x1A8u32, 0x1A4, 0x1A0, 0x1B8, 0x1B4, 0x1B0] {
            core::ptr::write_unaligned((obj + off) as *mut u32, 0);
        }
        for off in [0xC0u32, 0xC4, 0xC8, 0xCC, 0xD0, 0xD4] {
            core::ptr::write_unaligned((obj + off) as *mut u32, 0);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(LIMITS_INIT, u32, obj);
        obj
    }
});
