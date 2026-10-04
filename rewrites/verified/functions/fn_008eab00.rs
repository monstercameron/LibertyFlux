// original: 0x008eab00 nearest_node_below_threshold
/// Find the best node of group `idx` under the running threshold.
///
/// Scans nodes `start..start+count` (from the +0xB04/+0xC04 arrays)
/// whose flag nibble is `0xB0`. Each candidate first passes a cheap
/// Manhattan-style
/// estimate, then a precise stubbed distance call; when the combined cost
/// beats `*thresh`, the threshold and the winning id are updated. Returns
/// `idx`.
export!(thiscall, rw_008eab00(
    this: *const u8,
    out_id: *mut u32,
    idx: u32,
    vec: *const f32,
    thresh: *mut f32,
) -> u32 {
    unsafe {
        let base = *(this.add((0x804 + idx * 4) as usize) as *const u32);
        if base == 0 {
            return idx;
        }
        let k: f32 = *global::<f32>(0xFE87A4);
        let kz: f32 = *global::<f32>(0xFE8720);
        let mask: u32 = *global::<u32>(0xFE8F80);
        let zw: f32 = *global::<f32>(0xFE8A94);
        let w2: f32 = *global::<f32>(0xFE87E8);
        let w3: f32 = *global::<f32>(0xFE87D0);
        let start = *(this.add((0xB04 + idx * 4) as usize) as *const i32);
        let count = *(this.add((0xC04 + idx * 4) as usize) as *const i32);
        let bound = start.wrapping_add(count);
        let bitand = |v: f32| f32::from_bits(v.to_bits() & mask);
        let mut i = start;
        while i < bound {
            let node = base.wrapping_add((i as u32).wrapping_mul(32));
            i += 1;
            if *((node + 0x1c) as *const u8) & 0xF0 != 0xB0 {
                continue;
            }
            let dy = bitand(
                ((node + 0x16) as *const i16).read_unaligned() as f32 * k
                    - *vec.add(1),
            );
            let dx = bitand(
                ((node + 0x14) as *const i16).read_unaligned() as f32 * k
                    - *vec,
            );
            let mut d = dy + dx;
            let dz = bitand(
                ((node + 0x18) as *const i16).read_unaligned() as f32 * kz
                    - *vec.add(2),
            );
            d = d + dz * zw;
            d = d * w2;
            if !(*thresh > d) {
                continue;
            }
            let s: f32 =
                callee_thiscall!(1, f32, this as u32, node, vec as u32);
            let cand = s * w3 + d;
            if !(*thresh > cand) {
                continue;
            }
            *thresh = cand;
            *out_id = (((i - 1) as u32) << 16) | idx;
        }
        idx
    }
});
