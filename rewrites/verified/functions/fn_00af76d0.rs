// original: 0x00AF76D0 veh_state_is_ready (proposed)

/// Test whether the object is in the ready state.
///
/// Returns 1 when the flag byte at `this + 0x9B` is clear. Otherwise the
/// state byte at `this + 0x9A` decides: 1 only when it holds exactly 5.
///
/// Original: 0x00AF76D0 (thiscall, no stack arguments, result in AL).
lf_checker_rt::export!(thiscall, rw_00AF76D0(this: u32) -> u8 {
    unsafe {
        const FLAG: u32 = 0x9B;
        const STATE: u32 = 0x9A;
        const READY_STATE: u8 = 5;
        if ((this + FLAG) as *const u8).read() == 0 {
            1
        } else {
            (((this + STATE) as *const u8).read() == READY_STATE) as u8
        }
    }
});
