// original: 0x00926BB0 input_slot_touch
/// Refresh input slot `a0` and bump its use count.
///
/// Index -1 does nothing. Index 0 runs the slot-0 refresher; other indexes
/// run one of two refreshers depending on whether the slot's flag word is
/// zero. Then the slot's counter is bumped, and when the slot's level is
/// positive the shared level cell for that level is bumped as well.
/// Returns the level shifted left by 8 when positive, else the level.
export!(cdecl, rw_00926BB0(a0: u32, a1: u32) -> u32 {
    unsafe {
        if a0 == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        if a0 == 0 {
            callee_cdecl!(1, u32, 0, a1);
            let level = *global::<i32>(0x119F1F0);
            let counter = global::<u32>(0x119F200);
            *counter = (*counter).wrapping_add(1);
            if level <= 0 {
                return level as u32;
            }
            let cell = relocated(0x11A0C00).wrapping_add((level as u32).wrapping_shl(8))
                as *mut u32;
            *cell = (*cell).wrapping_add(1);
            // The original shifts the level left by 8 to form the cell
            // address and returns that shifted value, not the level.
            return (level as u32).wrapping_shl(8);
        }
        let stride = a0.wrapping_mul(0x110);
        if *global::<u32>(0x119F100u32.wrapping_add(stride)) == 0 {
            callee_cdecl!(2, u32, a0, a1);
        } else {
            callee_cdecl!(3, u32, a0);
        }
        let level = *(relocated(0x119F1F0u32.wrapping_add(stride)) as *const i32);
        let counter = relocated(0x119F200u32.wrapping_add(stride)) as *mut u32;
        *counter = (*counter).wrapping_add(1);
        if level <= 0 {
            return level as u32;
        }
        let cell = relocated(0x11A0C00).wrapping_add((level as u32).wrapping_shl(8))
            as *mut u32;
        *cell = (*cell).wrapping_add(1);
        (level as u32).wrapping_shl(8)
    }
});
