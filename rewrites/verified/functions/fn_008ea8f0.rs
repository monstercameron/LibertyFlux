// original: 0x008ea8f0 PathfindManagerParkNode
/// Pick a parking node inside an axis-aligned box (round-robin).
///
/// Scans 64 node groups on `this` (parallel arrays: bases at +0x804,
/// counts at +0xB04). A node matches when its scaled position is strictly inside the
/// box `(x_lo,x_hi) x (y_lo,y_hi) x (z_lo,z_hi)` and its flag byte has the
/// parked nibble `0x20`. Writes the match selected by the global
/// round-robin counter into `out` (or the first match when the counter
/// overshoots, or zeros when nothing matches), advances the counter, and
/// returns `out`.
export!(thiscall, rw_008ea8f0(
    this: *const u8,
    out: *mut f32,
    x_lo: f32, x_hi: f32,
    y_lo: f32, y_hi: f32,
    z_lo: f32, z_hi: f32,
) -> u32 {
    unsafe {
        const GROUPS: u32 = 64;
        const BASES: u32 = 0x804;
        const COUNTS: u32 = 0xB04;
        const NODE_HEAD: u32 = 0x14;
        const NODE_STRIDE: u32 = 0x20;
        let k: f32 = *global::<f32>(0xFE87A4);
        let kz: f32 = *global::<f32>(0xFE8720);
        let sel: u32 = *global::<u32>(0x117E6C0);
        let mut matched: u32 = 0;
        let mut first = [0.0f32; 3];
        let mut picked = [0.0f32; 3];
        let mut have_first = false;
        let mut have_picked = false;
        let mut i = 0u32;
        while i < GROUPS {
            let base = *(this.add((BASES + i * 4) as usize) as *const u32);
            let count =
                *(this.add((COUNTS + i * 4) as usize) as *const i32);
            i += 1;
            if base == 0 {
                continue;
            }
            if count <= 0 {
                continue;
            }
            let base = base.wrapping_add(NODE_HEAD);
            let mut j = 0i32;
            while j < count {
                let node = base.wrapping_add((j as u32).wrapping_mul(NODE_STRIDE));
                j += 1;
                let x = ((node as *const i16).read_unaligned() as f32) * k;
                if !(x > x_lo && x_hi > x) {
                    continue;
                }
                let y = (((node + 2) as *const i16).read_unaligned() as f32) * k;
                if !(y > y_lo && y_hi > y) {
                    continue;
                }
                let z = (((node + 4) as *const i16).read_unaligned() as f32) * kz;
                if !(z > z_lo && z_hi > z) {
                    continue;
                }
                if *((node + 8) as *const u8) & 0xF0 != 0x20 {
                    continue;
                }
                if matched == 0 {
                    first = [x, y, z];
                    have_first = true;
                }
                if matched == sel {
                    picked = [x, y, z];
                    have_picked = true;
                }
                matched += 1;
            }
        }
        let next = sel.wrapping_add(1);
        *global::<u32>(0x117E6C0) = if (next as i32) >= (matched as i32) {
            0
        } else {
            next
        };
        let src = if !have_first {
            [0.0f32; 3]
        } else if have_picked {
            picked
        } else {
            first
        };
        *out = src[0];
        *out.add(1) = src[1];
        *out.add(2) = src[2];
        out as u32
    }
});
