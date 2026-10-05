// original: 0x00a3ae70 vehicle_rate_in_window (proposed)

/// Test whether a rate float sits strictly between -4000 and 0.
///
/// Loads the float at `*(obj) + 0x1090 + 0x1c` and answers 1 exactly when
/// it is ordered-less than 0 and ordered-greater than -4000 (a NaN answers
/// 0). Thiscall, no stack arguments, returns AL.
lf_checker_rt::export!(thiscall, rw_00a3ae70(obj: u32) -> u32 {
    unsafe {
        const BASE_OFF: u32 = 0x1090;
        const SLOT: u32 = 0x1C;
        const LO: f32 = f32::from_bits(0xC57A_0000); // -4000.0
        let inner = core::ptr::read_unaligned(obj as *const u32);
        let v = f32::from_bits(core::ptr::read_unaligned((inner + BASE_OFF + SLOT) as *const u32));
        ((v < 0.0) & (v > LO)) as u32
    }
});
