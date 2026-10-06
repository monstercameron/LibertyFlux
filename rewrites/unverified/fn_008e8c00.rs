// original: 0x008e8c00 search_best_scored_node
/// Search the 64 node tables for the best-scoring visible node.
///
/// Normalise the `(fx, fy)` direction (a zero vector keeps `fy` and uses 1.0
/// for x), then score every node of every live table as
/// `|dx| + |dy| + 3 * |dz|` against `vec`. Nodes whose flag byte has any of
/// the top four bits set are skipped. A node beating the running best (from
/// 10000.0) is refined through the pair-scoring helper (thiscall/3, stubbed,
/// float answer); when `score - (answer - 1) * 40` still wins, the output
/// word becomes `(node << 16) | table`. Returns the output pointer.
export!(thiscall, rw_008e8c00(
    this: *mut u8,
    out: *mut u32,
    vec: *const u8,
    fx: u32,
    fy: u32,
) -> u32 {
    unsafe {
        let fx = f32::from_bits(fx);
        let fy = f32::from_bits(fy);
        let mut len = fx * fx;
        len += fy * fy;
        len = len.sqrt();
        let (dirx, diry) = if len == 0.0 {
            (1.0f32, fy)
        } else {
            let inv = 1.0 / len;
            (inv * fx, inv * fy)
        };
        write_unaligned(out, 0xFFFFFFFFu32);
        let vx = f32::from_bits(read_unaligned(vec as *const u32));
        let vy = f32::from_bits(read_unaligned(vec.add(4) as *const u32));
        let vz = f32::from_bits(read_unaligned(vec.add(8) as *const u32));
        let mut best = 10000.0f32;
        let mut i = 0u32;
        while i < 64 {
            let slot = this.add(0x804 + i as usize * 4) as *const u32;
            let table = read_unaligned(slot) as *const u8;
            if table as u32 != 0 {
                let count = read_unaligned(
                    (slot as *const u8).add(0x300) as *const i32,
                );
                let mut j = 0i32;
                while j < count {
                    let elem = table.add(j as usize * 0x20);
                    if read_unaligned::<u8>(elem.add(0x1C)) & 0xF0 == 0 {
                        let mut dy =
                            (read_unaligned(elem.add(0x16) as *const i16)
                                as f32)
                                * 0.125;
                        dy -= vy;
                        let mut dx =
                            (read_unaligned(elem.add(0x14) as *const i16)
                                as f32)
                                * 0.125;
                        dx -= vx;
                        let mut dz =
                            (read_unaligned(elem.add(0x18) as *const i16)
                                as f32)
                                * 0.015625;
                        dz -= vz;
                        let mut score = dy.abs() + dx.abs();
                        score += dz.abs() * 3.0;
                        if best > score {
                            let answer = callee_thiscall!(
                                1,
                                f32,
                                this as u32,
                                elem as u32,
                                dirx.to_bits(),
                                diry.to_bits()
                            );
                            let adjusted = score - (answer - 1.0) * 40.0;
                            if best > adjusted {
                                best = adjusted;
                                write_unaligned(
                                    out,
                                    ((j as u32) << 16) | i,
                                );
                            }
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        out as u32
    }
});
