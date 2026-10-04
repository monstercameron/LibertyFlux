// original: 0x00924D90 input_active_check
/// Report whether input is active: three gates in series.
///
/// Modes 1-3 of global `0x01045538` fail the first gate. Otherwise a helper
/// pair produces a 64-bit value that must be positive or reach 0x1C200000,
/// global `0x01160EA8` must be nonzero, and the level check must agree.
export!(cdecl, rw_00924D90() -> u32 {
    unsafe {
        let mut live = true;
        match *global::<u32>(0x1045538) {
            1 | 2 | 3 => live = false,
            _ => {
                let helper = callee_stdcall!(1, u32, 0);
                let v = callee_thiscall!(2, u64, helper);
                let (lo, hi) = (v as u32, (v >> 32) as i32);
                if hi > 0 {
                } else if hi < 0 {
                    live = false;
                } else if lo < 0x1C200000 {
                    live = false;
                }
            }
        }
        if !live {
            return 0;
        }
        if *global::<u32>(0x1160EA8) == 0 {
            return 0;
        }
        if callee_cdecl!(3, u32,) & 0xFF == 0 {
            0
        } else {
            1
        }
    }
});
