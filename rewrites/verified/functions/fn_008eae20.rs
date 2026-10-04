// original: 0x008eae20 nearest_ids_two_track
/// Track the two nearest node ids to a 2D point.
///
/// Scans all 64 groups; nodes with a zero id are skipped. Keeps the best
/// id in `out1` and the runner-up in `out2`, demoting the old best when a
/// new winner from a different node beats it. Clears `out2` when even the
/// runner-up distance exceeds the cap. Returns `out2`.
export!(thiscall, rw_008eae20(
    this: *const u8,
    vec: *const f32,
    out1: *mut u32,
    out2: *mut u32,
) -> u32 {
    unsafe {
        let k: f32 = *global::<f32>(0xFE87A4);
        *out1 = 0;
        *out2 = 0;
        let mut best1: f32 = *global::<f32>(0xE833D8);
        let mut best2 = best1;
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
                let id = *((node + 0xc) as *const u32);
                if id == 0 {
                    continue;
                }
                let px = ((node + 0x14) as *const i16).read_unaligned()
                    as f32
                    * k;
                let py = ((node + 0x16) as *const i16).read_unaligned()
                    as f32
                    * k;
                let dx = *vec - px;
                let dy = *vec.add(1) - py;
                let d = (dx * dx + dy * dy).sqrt();
                if best1 > d {
                    if id == *out1 {
                        best1 = d;
                    } else {
                        *out2 = *out1;
                        *out1 = id;
                        best2 = best1;
                        best1 = d;
                    }
                } else {
                    if id == *out1 {
                        continue;
                    }
                    if best2 > d {
                        *out2 = id;
                        best2 = d;
                    }
                }
            }
        }
        if best2 > *global::<f32>(0xFE8B5C) {
            *out2 = 0;
        }
        out2 as u32
    }
});
