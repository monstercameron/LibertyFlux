// original: 0x00862310 physics_is_motion_candidate

/// Returns the original's one-byte eligibility result for the object in ECX.
/// A pair of exact mode values suppresses the result immediately. Otherwise
/// the object's active bit and two ordered float comparisons must pass, a
/// second flag bit must be set, and the state word at the mode record must be
/// zero. The COMISS comparisons make unordered float values fail the test.
///
/// This function has no outgoing calls or writes and uses no stack arguments.
lf_checker_rt::export!(thiscall, rw_00862310(this: u32) -> u32 {
    unsafe {
        const STATE_RECORD: u32 = 0x000c;
        const MODE_A: u32 = 0x0310;
        const MODE_B: u32 = 0x0314;
        const ACTIVE_FLAGS: u32 = 0x00b5;
        const ACTIVE_BIT: u8 = 0x01;
        const STATE_FLAGS: u32 = 0x027c;
        const MOTION_BIT: u8 = 0x04;
        const CURRENT_SPEED: u32 = 0x0144;
        const SPEED_LIMIT: u32 = 0x0080;
        const MOTION_VALUE: u32 = 0x006c;
        const MOTION_LIMIT: u32 = 0x014c;

        #[inline(always)]
        unsafe fn read_word(base: u32, offset: u32) -> u32 {
            unsafe { ((base.wrapping_add(offset)) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_byte(base: u32, offset: u32) -> u8 {
            unsafe { ((base.wrapping_add(offset)) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_float(base: u32, offset: u32) -> f32 {
            unsafe { f32::from_bits(read_word(base, offset)) }
        }

        let state = read_word(this, STATE_RECORD);
        let mode_a = read_word(state, MODE_A);
        let mode_b = read_word(state, MODE_B);
        if mode_a == 1 && mode_b == 1 {
            return 0;
        }
        if read_byte(this, ACTIVE_FLAGS) & ACTIVE_BIT == 0 {
            return 0;
        }
        if !(read_float(this, CURRENT_SPEED) > read_float(this, SPEED_LIMIT)) {
            return 0;
        }
        if read_byte(this, STATE_FLAGS) & MOTION_BIT == 0 {
            return 0;
        }
        if !(read_float(this, MOTION_VALUE) > read_float(this, MOTION_LIMIT)) {
            return 0;
        }
        if mode_b != 0 {
            return 0;
        }
        1
    }
});
