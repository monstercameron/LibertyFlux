// original: 0x00C0E880 UIMusicContainer::vf108
/// Notify virtual slot 9 with a zero argument. When the nested state exists,
/// write the fixed gray value to its owner record, call its active-state
/// method, and call two more nested methods only when the returned low byte
/// is nonzero. Then look up the montage timer through the singleton and
/// return the timer-format helper's result. `this` is in ECX; every callee is
/// represented by a scripted call stand-in in the checker contract.
lf_checker_rt::export!(thiscall, rw_c0e880(this: u32) -> u32 {
    unsafe {
        const NESTED_STATE: u32 = 0x200;
        const NESTED_OWNER: u32 = 0x1fc;
        const NESTED_COLOUR: u32 = 0x1e0;
        const GRAY_COLOUR: u32 = 0xff79_7979;
        const TIMER_SINGLETON: u32 = 0x0198_1a4c;
        const TIMER_NAME: u32 = 0x00ec_3720;

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, 0);
        let nested_state = ((this + NESTED_STATE) as *const u32).read_unaligned();
        if nested_state != 0 {
            let nested_owner = ((nested_state + NESTED_OWNER) as *const u32).read_unaligned();
            ((nested_owner + NESTED_COLOUR) as *mut u32).write_unaligned(GRAY_COLOUR);

            let active = lf_checker_rt::callee_thiscall!(2, u32, nested_state);
            if (active as u8) != 0 {
                let _ = lf_checker_rt::callee_thiscall!(3, u32, nested_state);
                let _ = lf_checker_rt::callee_thiscall!(4, u32, nested_state, 0);
            }
        }

        let timer = lf_checker_rt::callee_thiscall!(
            5, u32,
            lf_checker_rt::relocated(TIMER_SINGLETON),
            lf_checker_rt::relocated(TIMER_NAME),
            0
        );
        lf_checker_rt::callee_thiscall!(6, u32, timer)
    }
});
