// original: 0x00ae0d10 ui_update_gauge_value
/// Recompute this object's gauge byte from its inputs and a peer object.
///
/// Blends the base value at 0x10 (scaled by `scale`) with the peer-or-own
/// value at 0xc through a clamped rational curve, optionally notifies the
/// worker with a 0-255 level, then lowers the stored byte at 0x20 towards
/// the new level. Returns the new level byte. All arithmetic is single
/// precision in program order; callers keep inputs finite so every
/// comparison is ordered. Objects whose mode byte at 0x25 is 2 or more
/// return immediately with an undefined value and are out of scope.
export!(thiscall, rw_00ae0d10(this: *mut u8, peer: *mut u8, scale_bits: u32, gain_bits: u32) -> u32 {
    unsafe {
        const K_BASE: u32 = 0x0103F714; // 20.0, shared tunable
        const K_SPAN: u32 = 0x00FE8BD0; // 150.0
        const K_SLOPE: u32 = 0x00FE877C; // 1/15
        const K_FLOOR: u32 = 0x00FE8B08; // 10.0
        const K_ONE: u32 = 0x00FE88E8; // 1.0
        const K_LEVEL: u32 = 0x00FE8C08; // 255.0
        const K_STEP: u32 = 0x00EA4D28; // 1/510
        const K_HALF: u32 = 0x00FE8830; // 0.5
        const K_TWO: u32 = 0x00FE8A24; // 2.0
        let t = this;
        let c_base = *global::<f32>(K_BASE);
        let other = if peer.is_null() { t } else { peer };
        let mut x0 = *(t.add(0x10) as *const f32) * f32::from_bits(scale_bits);
        let x4 = *(other.add(0x0c) as *const f32);
        let mut x2 = c_base;
        if peer.is_null() {
            let mut x3 = *(t.add(0x10) as *const f32);
            if !(x0 > x3) {
                x3 = x0;
            }
            if x3 > *global::<f32>(K_SPAN) {
                x2 = x3 * *global::<f32>(K_SLOPE) + *global::<f32>(K_FLOOR);
            }
            if *t.add(0x29) & 0x0c != 0 {
                x0 = x0 * f32::from_bits(gain_bits);
            }
        }
        let mut x1 = (c_base + x0 - x4) / x2;
        let c_one = *global::<f32>(K_ONE);
        if !(c_one > x1) {
            x1 = c_one;
        }
        let mut span = *global::<f32>(K_LEVEL);
        if !peer.is_null() {
            *peer.add(0x29) |= 2;
            let stored = *t.add(0x20);
            if stored != 0xff {
                let cap = (stored as f32) * *global::<f32>(K_STEP);
                if !(cap > x1) {
                    x1 = cap;
                }
            }
            if !(*global::<f32>(K_HALF) <= x1) {
                x1 = x1 * *global::<f32>(K_TWO);
                *peer.add(0x2a) |= 0x10;
            } else {
                let mut level = c_one - x1;
                level = level * *global::<f32>(K_TWO);
                level = level * span;
                let n = level as i32;
                callee_cdecl!(1, u32, (peer as u32).wrapping_add(0x21), n as u32);
                x1 = c_one;
                span = *global::<f32>(K_LEVEL);
            }
        }
        let old = *t.add(0x20);
        let c = (x1 * span) as i32;
        let low = (c & 0xFF) as u8;
        *t.add(0x20) = if low < old { low } else { old };
        low as u32
    }
});
