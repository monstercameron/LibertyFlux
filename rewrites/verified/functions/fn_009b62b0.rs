// original: 0x009B62B0 script_position_countdown_gate

/// Update the shared script-position countdown and return its ready state.
/// The function first queries the manager's two position components through a
/// direct helper call. A nonzero AL result resets the global dword counter to
/// 15 and returns one. Otherwise the counter is interpreted as signed: values
/// greater than zero are decremented, then the result is true when nonzero;
/// zero remains zero and returns false, while negative values remain unchanged
/// and return true.
lf_checker_rt::export!(cdecl, rw_009b62b0() -> u32 {
    unsafe {
        const COUNTER_GLOBAL: u32 = 0x0103_9294;
        const READY_HELPER: u32 = 1;
        const RESET_COUNT: u32 = 15;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let counter_slot = lf_checker_rt::relocated(COUNTER_GLOBAL);
        let ready = lf_checker_rt::callee_cdecl!(READY_HELPER, u8,);
        if ready != 0 {
            write_u32(counter_slot, RESET_COUNT);
            1
        } else {
            let count = read_u32(counter_slot) as i32;
            if count > 0 {
                let remaining = count - 1;
                write_u32(counter_slot, remaining as u32);
                u32::from(remaining != 0)
            } else {
                u32::from(count != 0)
            }
        }
    }
});
