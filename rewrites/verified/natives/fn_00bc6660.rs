// original: 0x00BC6660 IS_CAR_A_MISSION_CAR
/// F11 IS_CAR_A_MISSION_CAR: 1 arg, low byte of answer.
export!(cdecl, rn10_is_car_a_mission_car(ctx: *const u32) -> u32 {
    unsafe {
        let (ret, args) = ctx_parts(ctx);
        let answer: u32 = callee_cdecl!(2, u32, *args);
        *ret = answer & 0xFF;
        ret as u32
    }
});
