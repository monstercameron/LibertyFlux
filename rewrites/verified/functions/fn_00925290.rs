// original: 0x00925290 input_ready_check
/// Report whether input is ready: gated counter plus a 64-bit threshold.
///
/// Returns false unless global `0x01160EAC` is at least 2. Modes 1-3 of
/// global `0x01045538` also report false; otherwise a helper pair produces
/// a 64-bit value that must be positive, or zero-extended past 0x16800000.
export!(cdecl, rw_00925290() -> u32 {
    unsafe {
        if *global::<i32>(0x1160EAC) < 2 {
            return 0;
        }
        let ready = match *global::<u32>(0x1045538) {
            1 | 2 | 3 => false,
            _ => {
                let helper = callee_stdcall!(1, u32, 0);
                let v = callee_thiscall!(2, u64, helper);
                let (lo, hi) = (v as u32, (v >> 32) as i32);
                if hi > 0 {
                    true
                } else if hi < 0 {
                    false
                } else {
                    lo >= 0x16800000
                }
            }
        };
        u32::from(ready)
    }
});
