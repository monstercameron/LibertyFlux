// original: 0x00b86eb0 IS_FOLLOW_VEHICLE_CAM_OFFSET_ACTIVE

/// Native handler `IS_FOLLOW_VEHICLE_CAM_OFFSET_ACTIVE`.
///
/// Report whether the follow-vehicle camera offset is active.
/// Forwards 0 argument(s) to the engine and stores the low byte
/// of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b86eb0(ctx: *const u32) -> u32 {
    unsafe {
        // The original keeps only the low byte of the answer.
        let answer = callee_cdecl!(1, u32,);
        *(*ctx as *mut u32) = answer & 0xFF;
        0
    }
});
