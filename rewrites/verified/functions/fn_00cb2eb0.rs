// original: 0x00CB2EB0 CTaskComplexMoveGoToPointStandStillAchieveHeading::vf19

/// Create the go-to-point subtask for this complex task.
///
/// `this` is the complex task object, `ped` the ped the task runs on. The
/// method forwards both to the subtask factory (callee 1) with the fixed
/// subtask id `GO_TO_POINT` (0x387) and returns whatever the factory
/// returns. No memory is read or written besides the call itself.
///
/// Original: 0x00CB2EB0 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB2EB0(this: u32, ped: u32) -> u32 {
    unsafe {
        const CREATE_SUB: u32 = 1;
        const GO_TO_POINT: u32 = 0x387;
        lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, ped, GO_TO_POINT)
    }
});
