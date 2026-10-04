// original: 0x00bd7760 CAN_REGISTER_MISSION_VEHICLE
// rw_can_register_mission_vehicle: native CAN_REGISTER_MISSION_VEHICLE (handler 0x00BD7760).
//
// No-arg query: calls the mission-vehicle registrar check, stores the low byte of the answer in the return slot.
export!(cdecl, rw_can_register_mission_vehicle(ctx: *mut u32) -> u32 {
    unsafe {
        let ans = callee_cdecl!(1, u32,);
        *(*ctx as *mut u32) = ans & 0xFF;
        ans
    }
});
