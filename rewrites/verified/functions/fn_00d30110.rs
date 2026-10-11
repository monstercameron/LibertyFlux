// original: 0x00D30110 CTaskSimpleMovePathfindProblem::vf5

/// When the second stack argument is zero, initialize three task fields:
/// copy the current-time word from the engine global to offset 0x3c, write
/// 0xffff_ffff at 0x40, and set the byte at 0x44 to one. For every value of
/// that argument, return one in AL. This is a thiscall method with three
/// 32-bit stack arguments; the callee removes all twelve bytes.
lf_checker_rt::export!(thiscall, rw_00D30110(this: u32, _first: u32, mode: u32, _third: u32) -> u8 {
    const CURRENT_TIME_VA: u32 = 0x11735b4;
    const TIME_FIELD: u32 = 0x3c;
    const SENTINEL_FIELD: u32 = 0x40;
    const READY_FIELD: u32 = 0x44;
    if mode == 0 {
        let time_bits = unsafe { crate::read_u32(lf_checker_rt::global::<u32>(CURRENT_TIME_VA) as u32) };
        unsafe {
            crate::write_u32(this.wrapping_add(TIME_FIELD), time_bits);
            crate::write_u32(this.wrapping_add(SENTINEL_FIELD), u32::MAX);
            crate::write_u8(this.wrapping_add(READY_FIELD), 1);
        }
    }
    1
});
