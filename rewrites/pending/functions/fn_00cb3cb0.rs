// original: 0x00CB3CB0 CTaskComplexMoveGoToPointStandStillAchieveHeading::vf18

/// Replace a finished go-to-point subtask with the follow-up subtask.
///
/// `this` is the complex task, `ped` the ped. The type of the current
/// subtask (`[this+8]`, virtual slot 3, callee 1) decides: while it is one
/// of the two go-to-point ids (0x386, 0x387) a follow-up subtask (id 0x516)
/// is created through the factory (callee 2) and returned; any other type
/// keeps the current subtask and yields null.
///
/// Original: 0x00CB3CB0 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB3CB0(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 8;
        const GET_TYPE_SLOT: u32 = 0x0c;
        const CREATE_SUB: u32 = 2;
        const GO_TO_POINT_A: u32 = 0x386;
        const GO_TO_POINT_B: u32 = 0x387;
        const FOLLOW_UP: u32 = 0x516;
        let sub = (this.wrapping_add(SUBTASK) as *const u32).read_unaligned();
        let vtable = (sub as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(GET_TYPE_SLOT) as *const u32).read_unaligned();
        let get_type: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        match get_type(sub) {
            GO_TO_POINT_A | GO_TO_POINT_B => {
                lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, ped, FOLLOW_UP)
            }
            _ => 0,
        }
    }
});
