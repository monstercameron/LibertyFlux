// original: 0x008ed7e0 top_n_nearest_collector
/// Collect the ids of the N nearest matching nodes.
///
/// Scans all 64 groups for nodes that pass the parity, gate and type
/// filters and lie within `bound` of `vec`, keeping the closest N in
/// `buf` (with N+1 slots: the shift briefly uses one past the end).
/// Returns the loader routine's answer (the stubbed security cookie
/// check yields 0).
export!(thiscall, rw_008ed7e0(
    this: *const u8,
    vec: *const f32,
    n: u32,
    buf: *mut u32,
    bound: f32,
    f1: u32,
    f2: u32,
) -> u32 {
    unsafe {
        let nn = n as i32;
        let mut dist = [f32::from_bits(0x501502F9); 9];
        let mut k = 0u32;
        while k < n {
            dist[k as usize] = f32::from_bits(0x501502F9);
            *buf.add(k as usize) = 0xFFFFFFFF;
            k += 1;
        }
        let kk: f32 = *global::<f32>(0xFE87A4);
        let kz: f32 = *global::<f32>(0xFE8720);
        let mut i = 0u32;
        while i < 64 {
            let base = *(this.add((0x804 + i * 4) as usize) as *const u32);
            let count =
                *(this.add((0xB04 + i * 4) as usize) as *const i32);
            i += 1;
            if base == 0 {
                continue;
            }
            if count <= 0 {
                continue;
            }
            let mut j = 0i32;
            while j < count {
                let node =
                    base.wrapping_add((j as u32).wrapping_mul(0x20));
                j += 1;
                let par = (*((node + 0x1f) as *const u8) >> 1) & 1;
                if (par as u32) != f2 & 0xFF {
                    continue;
                }
                if f1 & 0xFF != 0
                    && *((node + 0x1e) as *const u8) & 0x80 != 0
                {
                    continue;
                }
                let nib = *((node + 0x1c) as *const u8) >> 4;
                if nib == 7 || nib == 8 {
                    continue;
                }
                let dx = ((node + 0x14) as *const i16).read_unaligned()
                    as f32
                    * kk
                    - *vec;
                let dy = ((node + 0x16) as *const i16).read_unaligned()
                    as f32
                    * kk
                    - *vec.add(1);
                let dz = ((node + 0x18) as *const i16).read_unaligned()
                    as f32
                    * kz
                    - *vec.add(2);
                let d = (dy * dy + dx * dx + dz * dz).sqrt();
                if !(bound > d) {
                    continue;
                }
                let mut pos = nn;
                let mut q = nn;
                while q > 0 {
                    if !(dist[(q - 1) as usize] > d) {
                        pos = q;
                        break;
                    }
                    q -= 1;
                    if q == 0 {
                        pos = 0;
                    }
                }
                if pos >= nn {
                    continue;
                }
                let mut m = nn - 1;
                while m >= pos {
                    dist[(m + 1) as usize] = dist[m as usize];
                    *buf.add((m + 1) as usize) = *buf.add(m as usize);
                    m -= 1;
                }
                dist[pos as usize] = d;
                *buf.add(pos as usize) = *((node + 8) as *const u32);
            }
        }
        callee_cdecl!(1, u32,)
    }
});
