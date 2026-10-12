// original: 0x00A18CD0 CCamFollowVehicle::maybe_solve_mode3
/// Calls the mode-three solver only when the camera's mode byte is 3. It
/// forwards the five incoming stack words unchanged in thiscall order and
/// returns the solver's low byte; other modes return zero without a call.
lf_checker_rt::export!(thiscall, rw_00A18CD0(camera: u32, arg0_bits: u32, arg1_bits: u32, arg2_bits: u32, arg3_bits: u32, arg4_bits: u32) -> u8 {
    unsafe {
        const CAMERA_MODE: u32 = 0x22E;
        if (camera.wrapping_add(CAMERA_MODE) as *const u8).read() != 3 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(1, u32, camera, arg0_bits, arg1_bits, arg2_bits, arg3_bits, arg4_bits) as u8
    }
});
