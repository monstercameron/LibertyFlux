// original: 0x009B6370 script_read_component_angle

/// Read a script component angle after checking its action and parameter state.
/// The thiscall inputs are a component object in ECX, an expected action code
/// on the stack, and a byte-valued wrap flag. A null action, a mismatched
/// virtual action code, or parameter flag bit two returns the component's
/// fallback angle at `+0x90`. Otherwise a helper supplies the angle. When the
/// wrap flag is nonzero and the action state at `+0x38f` equals three, the
/// routine adds π and subtracts 2π only when the shifted value is greater than
/// π; equality and unordered comparisons keep the shifted value. The final
/// float is returned in ST0. The original also uses its first stack argument
/// slot for this temporary; the contract compares the same value through ST0.
lf_checker_rt::export!(thiscall, rw_009b6370(this: u32, expected_action: u32, wrap_flag: u32) -> f32 {
    unsafe {
        const MANAGER_GLOBAL: u32 = 0x0103_e49c;
        const ACTION_STATE: u32 = 0x38f;
        const PARAMETER_FLAGS: u32 = 0x13c;
        const COMPONENT_ANGLE: u32 = 0x10;
        const FALLBACK_ANGLE: u32 = 0x90;
        const ANGLE_SHIFT: f32 = f32::from_bits(0x4049_0fdb);
        const FULL_TURN: f32 = f32::from_bits(0x40c9_0fdb);
        const ACTION_LOOKUP: u32 = 1;
        const ACTION_CODE: u32 = 2;
        const PARAMETER_LOOKUP: u32 = 3;
        const ANGLE_LOOKUP: u32 = 4;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }
        #[inline(never)]
        fn add_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(never)]
        fn sub_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        let manager = read_u32(lf_checker_rt::relocated(MANAGER_GLOBAL));
        let action = lf_checker_rt::callee_thiscall!(ACTION_LOOKUP, u32, manager, 1u32, 0u32);
        if action == 0 {
            return read_f32(this.wrapping_add(FALLBACK_ANGLE));
        }

        let action_code = lf_checker_rt::callee_thiscall!(ACTION_CODE, u32, action);
        if action_code != expected_action {
            return read_f32(this.wrapping_add(FALLBACK_ANGLE));
        }

        let parameters = lf_checker_rt::callee_thiscall!(PARAMETER_LOOKUP, u32, manager, 0x27u32, 0u32);
        if read_u8(parameters.wrapping_add(PARAMETER_FLAGS)) & 4 != 0 {
            return read_f32(this.wrapping_add(FALLBACK_ANGLE));
        }

        let angle = lf_checker_rt::callee_thiscall!(
            ANGLE_LOOKUP,
            f32,
            action.wrapping_add(COMPONENT_ANGLE)
        );
        if wrap_flag as u8 != 0 && read_u8(action.wrapping_add(ACTION_STATE)) == 3 {
            let shifted = add_f32(angle, ANGLE_SHIFT);
            if shifted > ANGLE_SHIFT {
                sub_f32(shifted, FULL_TURN)
            } else {
                shifted
            }
        } else {
            angle
        }
    }
});
