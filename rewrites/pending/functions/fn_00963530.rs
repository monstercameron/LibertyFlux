// original: 0x00963530 select_measure_and_scale
/// Select a measurement by kind and store its scaled value.
///
/// Kind 0 stores 0 in both slots; kinds 3 through 6 store the kind and a
/// fixed scale (15, 50, 75, 99). Kind 1 resolves a helper value, adds the
/// second argument, looks the entry up through the second helper, and
/// derives the value from the entry's fourth word minus the first base
/// plus one. Kind 2 uses the second argument directly. Both computed kinds
/// subtract the second base, store the value, and also store it as an
/// unsigned float. Any other kind stores nothing.
export!(cdecl, rw_00963530(kind: u32, arg1: u32) -> u32 {
    unsafe {
        const SCALE_SLOT: u32 = 0x11F70D0;
        const KIND_SLOT: u32 = 0x11F70D4;
        const FLOAT_SLOT: u32 = 0x11F7054;
        const BASE_A: u32 = 0x11F7028;
        const BASE_B: u32 = 0x11F7030;
        if kind == 0 {
            *(relocated(KIND_SLOT) as *mut u32) = 0;
            *(relocated(SCALE_SLOT) as *mut u32) = 0;
            return 0;
        }
        let fixed = match kind {
            3 => Some(0xFu32),
            4 => Some(0x32),
            5 => Some(0x4B),
            6 => Some(0x63),
            _ => None,
        };
        if let Some(scale) = fixed {
            *(relocated(KIND_SLOT) as *mut u32) = kind;
            *(relocated(SCALE_SLOT) as *mut u32) = scale;
            return 0;
        }
        let value = match kind {
            1 => {
                *(relocated(KIND_SLOT) as *mut u32) = 1;
                let helper: u32 = callee_cdecl!(1, u32,);
                let entry: u32 = callee_cdecl!(2, u32, helper.wrapping_add(arg1));
                let word = *((entry.wrapping_add(0x0C)) as *const u32);
                word
                    .wrapping_sub(*(relocated(BASE_A) as *const u32))
                    .wrapping_add(1)
            }
            2 => {
                *(relocated(KIND_SLOT) as *mut u32) = 2;
                arg1
            }
            _ => return 0,
        };
        let scaled = value.wrapping_sub(*(relocated(BASE_B) as *const u32));
        *(relocated(SCALE_SLOT) as *mut u32) = scaled;
        let adjust = if scaled & 0x8000_0000 != 0 {
            4294967296.0f64
        } else {
            0.0
        };
        *(relocated(FLOAT_SLOT) as *mut f32) = ((scaled as i32) as f64 + adjust) as f32;
        0
    }
});
