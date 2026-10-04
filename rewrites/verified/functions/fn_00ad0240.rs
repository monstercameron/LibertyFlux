// original: 0x00ad0240 audio_slot_update
/// Audio slot update with filtered level (original 0xAD0240).
///
/// `this` is the audio slot, `a0` a coefficient block, `a1` a vector block,
/// `a2` a control word whose low byte gates the update. When the gate byte
/// is zero the slot is reset through the reset helper and zeroed; otherwise
/// a filtered level is projected from the coefficient rows, clamped against
/// the slot rate, rate-shaped by the global band, flag-adjusted, limited,
/// and stored into the slot's level fields. The vector block is then copied
/// into both vector slots, the direction reset, and a pending entry released
/// when one is queued. Returns the helper answer on the reset path, the
/// release answer when an entry was queued, else the vector tail word.
export!(thiscall, rw_b45_0240(this: *mut u8, a0: *const u8, a1: *const u8, a2: u32) -> u32 {
    unsafe {
        if (a2 & 0xFF) == 0 {
            let ans: u32 = callee_thiscall!(2, u32, this as u32, 0);
            *(this.add(0x74) as *mut u32) = 0;
            *(this.add(0x70) as *mut u32) = 0;
            *(this.add(0x78) as *mut u32) = 0;
            return ans;
        }
        let c0 = *(a0.add(8) as *const f32);
        let c1 = *(a0.add(0x18) as *const f32);
        let c2 = *(a0.add(0x28) as *const f32);
        let mut t = c1 * *(this.add(0x64) as *const f32);
        t += c0 * *(this.add(0x60) as *const f32);
        t += c2 * *(this.add(0x68) as *const f32);
        t += *(a0.add(0x38) as *const f32);
        let mut s = c0 * *(this.add(0x50) as *const f32);
        s += c1 * *(this.add(0x54) as *const f32);
        s += c2 * *(this.add(0x58) as *const f32);
        s += *(a0.add(0x38) as *const f32);
        let rate = *(this.add(0x18) as *const f32);
        let mut x = *(a1.add(8) as *const f32);
        x -= t;
        x *= rate;
        x /= s - t;
        x = if rate > x { x } else { rate };
        let cutoff = *global::<f32>(0xFE8C58);
        let shape = *(this.add(0x160) as *const f32);
        let one = *global::<f32>(0xFE88E8);
        *(this.add(0x74) as *mut f32) = x;
        *(this.add(0x70) as *mut f32) = x;
        let base = if cutoff > shape {
            if !(0.0 >= shape) {
                let mut w = x / *(this.add(0x20) as *const f32);
                w = if w > one { one } else { w };
                let k = shape * *global::<f32>(0xFE86B4);
                let mut u = (one - k) * w;
                u = if 0.0 > u { 0.0 } else { u };
                (*(this.add(8) as *const f32) - *(this.add(0xC) as *const f32)) * u
            } else {
                *(this.add(8) as *const f32) - *(this.add(0xC) as *const f32)
            }
        } else {
            0.0
        };
        let mut flags = *(this.add(0x164) as *const u32);
        let mut adj = base;
        if flags & 0x0C000000 != 0 {
            let c = if (flags >> 27) & 1 != 0 {
                *global::<f32>(0xFE876C)
            } else {
                *global::<f32>(0xFE8748)
            };
            adj = c + base;
        }
        let lim = *(this.add(0x1C) as *const f32) + adj;
        x = if lim > x { x } else { lim };
        *(this.add(0x78) as *mut f32) = x;
        flags |= 1;
        *(this.add(0x164) as *mut u32) = flags;
        let w0 = *(a1 as *const u32);
        let w1 = *(a1.add(4) as *const u32);
        let w2 = *(a1.add(8) as *const u32);
        let w3 = *(a1.add(0xC) as *const u32);
        *(this.add(0x90) as *mut u32) = w0;
        *(this.add(0x94) as *mut u32) = w1;
        *(this.add(0x98) as *mut u32) = w2;
        *(this.add(0x9C) as *mut u32) = w3;
        *(this.add(0xA0) as *mut u32) = w0;
        *(this.add(0xA4) as *mut u32) = w1;
        *(this.add(0xA8) as *mut u32) = w2;
        *(this.add(0xAC) as *mut u32) = w3;
        *(this.add(0xC0) as *mut u32) = 0;
        *(this.add(0xC4) as *mut u32) = 0;
        *(this.add(0xC8) as *mut u32) = 0x3F800000;
        *(this.add(0xD8) as *mut u32) = 0;
        *(this.add(0xDC) as *mut u32) = 0;
        let gate = *(this.add(0xD0) as *const u32);
        let mut ans = w3;
        if gate != 0 {
            ans = callee_thiscall!(1, u32, gate, this.add(0xD0) as u32);
        }
        *(this.add(0xD0) as *mut u32) = 0;
        ans
    }
});
