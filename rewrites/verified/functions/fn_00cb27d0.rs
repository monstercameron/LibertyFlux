// original: 0x00CB27D0 CTaskComplexMoveBeInFormation::vf19

/// Keep the ped in formation, walking or running to its slot.
///
/// `this` is the complex task, `ped` the ped. The formation probe
/// (callee 1) runs first; when it says no, the result is null. Otherwise
/// the ped's movement mode (low nibble of `ped+0x1e2`) picks the subtask:
/// modes below 2 walk to the slot (0x3ae), the rest run (0x11a), created
/// through the factory (callee 2) as (id, ped).
///
/// Original: 0x00CB27D0 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB27D0(this: u32, ped: u32) -> u32 {
    unsafe {
        const IN_FORMATION: u32 = 1;
        const CREATE_SUB: u32 = 2;
        const MOVE_MODE: u32 = 0x1e2;
        const WALK_TO_SLOT: u32 = 0x3ae;
        const RUN_TO_SLOT: u32 = 0x11a;
        if lf_checker_rt::callee_thiscall!(IN_FORMATION, u32, this, ped) as u8 == 0 {
            return 0;
        }
        let mode = (ped.wrapping_add(MOVE_MODE) as *const u8).read_unaligned() & 0x0f;
        let id = if mode < 2 { WALK_TO_SLOT } else { RUN_TO_SLOT };
        lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, id, ped)
    }
});
