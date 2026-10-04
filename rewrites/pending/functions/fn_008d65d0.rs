// original: 0x008d65d0 update_facing_blend
// Blends an object's facing from a progress ratio: the ratio of elapsed to
// total counts is clamped into [0, 1] (NaN passes through), hitting the top
// also clears a saturated flag, and depending on the mode word the blend runs either
// directly or through a shaped mapping before dispatch. Returns the dispatch
// answer, or mode - 1 when the mode selects neither path.
export!(thiscall, rw_008d65d0(obj: *mut u8, flags: u32) -> u32 {
    unsafe {
        let base = *global::<u32>(0x0117_35D4);
        let start = *(obj.wrapping_add(0x10) as *const u32);
        let span = *(obj.wrapping_add(0x14) as *const i32);
        let ratio = (base.wrapping_sub(start) as i32) as f32 / (span as f32);
        // The ratio is clamped into [0, 1] (NaN passes through); exceeding 1
        // also clears the saturated flag.
        let r = if ratio > 1.0 {
            *(obj.wrapping_add(0x1C) as *mut u8) = 0;
            1.0
        } else if ratio < 0.0 {
            0.0
        } else {
            ratio
        };
        let mode = *(obj.wrapping_add(0x18) as *const u32);
        if mode == 0 {
            let r2 = r * *global::<f32>(0x00E8_1218);
            let mut x0 = *global::<f32>(0x00E8_121C) - r2;
            x0 *= *global::<f32>(0x00FE_8728);
            let t: u32 = callee_cdecl!(2, u32, x0.to_bits());
            x0 = f32::from_bits(t);
            x0 += *global::<f32>(0x00FE_88E8);
            x0 *= *global::<f32>(0x00FE_8830);
            callee_cdecl!(
                1,
                u32,
                obj as u32,
                (obj as u32).wrapping_add(8),
                x0.to_bits(),
                flags
            )
        } else if mode == 1 {
            callee_cdecl!(
                1,
                u32,
                obj as u32,
                (obj as u32).wrapping_add(8),
                r.to_bits(),
                flags
            )
        } else {
            mode.wrapping_sub(1)
        }
    }
});
