// original: 0x00b01600 classify_bounds
/// Classify a bounds rectangle against the unit square: 1 when a corner or
/// an edge sum is out of range above, 2 when one is out below, else 0.
export!(thiscall, rw_00b01600(this: u32) -> u32 {
    unsafe {
        let rect = callee_thiscall!(1, u32, this.wrapping_add(0x10)) as *const f32;
        let x0 = *rect;
        let y0 = *rect.add(1);
        let x1 = *rect.add(2);
        let y1 = *rect.add(3);
        let limit = *global::<f32>(0xfe88e8);
        let sx = x1 + x0;
        let sy = y1 + y0;
        // Each test mirrors one comiss+jae (negated strict less) or
        // comiss+jb (strict less) of the original, preserving NaN outcomes.
        if !(x0 < limit) {
            return 1;
        }
        if !(0.0 < sx) {
            return 1;
        }
        if !(y0 < limit) {
            return 1;
        }
        if !(0.0 < sy) {
            return 1;
        }
        if limit < x0 {
            return 2;
        }
        if limit < sx {
            return 2;
        }
        if x0 < 0.0 {
            return 2;
        }
        if sx < 0.0 {
            return 2;
        }
        if limit < y0 {
            return 2;
        }
        if limit < sy {
            return 2;
        }
        if y0 < 0.0 {
            return 2;
        }
        if sy < 0.0 {
            return 2;
        }
        0
    }
});
