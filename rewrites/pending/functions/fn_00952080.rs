// original: 0x00952080 state_byte_refresh
/// Refreshes the saved state byte through a gate and a probe call.
///
/// With a zero mode and a raised gate the saved byte is returned untouched.
/// Otherwise the outcome depends on a parameterless reset and, for nonzero
/// modes, a probe call: a rejected probe clears the mode before it is saved.
/// Returns the saved byte.
export!(cdecl, rw_00952080(mode: u32, param: u32) -> u32 {
    unsafe {
        const GATE: u32 = 0x0103_7868;
        const SAVED: u32 = 0x0103_76E9;
        const PROBE_TAG: u32 = 0x11;
        let flag = (mode & 0xFF) as u8;
        if flag == 0 {
            if *global::<u8>(GATE) != 0 {
                return *global::<u8>(SAVED) as u32;
            }
            if param & 0xFF == 0 {
                callee_cdecl!(1, u32,);
            }
            *global::<u8>(SAVED) = 0;
            return 0;
        }
        let ok = callee_cdecl!(2, u32, PROBE_TAG, param);
        if ok & 0xFF == 0 {
            callee_cdecl!(1, u32,);
            *global::<u8>(SAVED) = 0;
            return 0;
        }
        *global::<u8>(SAVED) = flag;
        flag as u32
    }
});
