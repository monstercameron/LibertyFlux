// original: 0x009BAFB0 CCamScriptInstruction_SetStockShake::vf2

/// Apply the stock camera shake named by this script instruction.
///
/// `this` points to a script instruction whose lookup key is at `+0x08` and
/// whose two shake parameters are at `+0x0C` and `+0x10`. A camera is looked
/// up through the camera manager. If the camera has no shake state at `+0x110`,
/// the setup helper creates it, an initialization helper runs, and the state
/// receives links to the camera plus a cleared flag and sentinel. The final
/// helper receives the state and the two parameters in order. A missing camera
/// is a no-op that returns zero; otherwise the final helper's result is
/// returned.
///
/// The original is a thiscall method with no stack arguments. All four helper
/// calls are intercepted by the contract; their object mutations and scripted
/// results are part of the compared call sequence.
lf_checker_rt::export!(thiscall, rw_009bafb0(this: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x0128_e400;
        const CAMERA_SHAKE: u32 = 0x110;
        const SHAKE_FLAG: u32 = 0x27c;
        const SHAKE_CAMERA_LINK: u32 = 0x270;
        const SHAKE_OWNER_LINK: u32 = 0x278;
        const SHAKE_SENTINEL: u32 = 0x280;
        const LOOKUP: u32 = 1;
        const CREATE_SHAKE: u32 = 2;
        const INITIALIZE_SHAKE: u32 = 3;
        const APPLY_SHAKE: u32 = 4;

        let lookup_key = (this.wrapping_add(8) as *const u32).read_unaligned();
        let camera = lf_checker_rt::callee_thiscall!(
            LOOKUP,
            u32,
            lf_checker_rt::relocated(MANAGER),
            lookup_key,
        );
        if camera == 0 {
            return 0;
        }

        let state_slot = camera.wrapping_add(CAMERA_SHAKE);
        let mut shake_state = (state_slot as *const u32).read_unaligned();
        if shake_state == 0 {
            let _ = lf_checker_rt::callee_thiscall!(CREATE_SHAKE, u32, camera);
            shake_state = (state_slot as *const u32).read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(INITIALIZE_SHAKE, u32, shake_state);

            (shake_state.wrapping_add(SHAKE_FLAG) as *mut u8).write(0);
            (shake_state.wrapping_add(SHAKE_CAMERA_LINK) as *mut u32)
                .write_unaligned(camera.wrapping_add(0x10));
            (shake_state.wrapping_add(SHAKE_OWNER_LINK) as *mut u32)
                .write_unaligned(camera);
            (shake_state.wrapping_add(SHAKE_SENTINEL) as *mut u32)
                .write_unaligned(u32::MAX);
        }

        let first_parameter = (this.wrapping_add(0x0c) as *const u32).read_unaligned();
        let second_parameter = (this.wrapping_add(0x10) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            APPLY_SHAKE,
            u32,
            shake_state,
            first_parameter,
            second_parameter,
        )
    }
});
