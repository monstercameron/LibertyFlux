// original: 0x00a89200 render_state_init_defaults
/// Initialise the render-state block to its default values.
///
/// Clears bit 0 of the byte at `this + 0x1481`, writes the default words
/// (float bit patterns and integer sentinels) across `this + 0x1430` to
/// `this + 0x1480`, and returns 1.
export!(thiscall, rw_00a89200(this_obj: u32) -> u32 {
    unsafe {
        let b = this_obj as usize;
        *((b.wrapping_add(0x1481)) as *mut u8) &= !1;
        *((b.wrapping_add(0x147c)) as *mut u32) = 0x14;
        *((b.wrapping_add(0x1478)) as *mut u32) = 0;
        *((b.wrapping_add(0x1474)) as *mut u32) = 0;
        *((b.wrapping_add(0x1460)) as *mut u32) = 0x3f400000;
        *((b.wrapping_add(0x1470)) as *mut u32) = 0;
        *((b.wrapping_add(0x1458)) as *mut u32) = 0x3a83126f;
        *((b.wrapping_add(0x1480)) as *mut u8) = 7;
        *((b.wrapping_add(0x145c)) as *mut u32) = 0x3c23d70b;
        *((b.wrapping_add(0x1454)) as *mut u32) = 0x3e967a10;
        *((b.wrapping_add(0x1450)) as *mut u32) = 0x3f800000;
        *((b.wrapping_add(0x1468)) as *mut u32) = 0;
        *((b.wrapping_add(0x146c)) as *mut u32) = 0xffffffff;
        *((b.wrapping_add(0x1438)) as *mut u32) = 0;
        *((b.wrapping_add(0x1434)) as *mut u32) = 0;
        *((b.wrapping_add(0x1430)) as *mut u32) = 0;
        *((b.wrapping_add(0x1448)) as *mut u32) = 0;
        *((b.wrapping_add(0x1444)) as *mut u32) = 0;
        *((b.wrapping_add(0x1440)) as *mut u32) = 0;
        *((b.wrapping_add(0x1464)) as *mut u16) = 0;
        1
    }
});
