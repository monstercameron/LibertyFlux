// original: 0x00b0b100 forward_position_to_ground_fix
/// Forward the anchor position and three arguments to the fixer.
///
/// Reads the anchor object behind this slot, passes its three position
/// words plus the three incoming arguments and the fixed friction word to
/// the shared position fixer. Returns nothing.
export!(thiscall, rw_00b0b100(this_ptr: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const ANCHOR_OFF: usize = 0x20;
        const FRICTION_BITS: u32 = 0x3f99_999a;
        let anchor = *((this_ptr as usize + ANCHOR_OFF) as *const u32);
        let x = *((anchor as usize + 0x30) as *const u32);
        let y = *((anchor as usize + 0x34) as *const u32);
        let z = *((anchor as usize + 0x38) as *const u32);
        callee_cdecl!(1, u32, x, y, z, a0, a1, a2, FRICTION_BITS);
        0
    }
});
