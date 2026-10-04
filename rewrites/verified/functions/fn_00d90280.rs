// original: 0x00d90280 audio_zone_match_test
/// Test whether any zone entry contains the point, returning 1 when none do.
///
/// Each of the `count` entries (read through the header at `hdr`) holds an
/// x/y centre and a radius; an entry matches when the point at `pt` falls
/// strictly inside its disc and its height differs from the entry's height
/// by less than the configured tolerance. An empty list matches nothing.
/// All arithmetic is single precision, exactly as the original evaluates it.
lf_rs89_rt::export!(cdecl, rw_00d90280(pt: u32, hdr: u32) -> u32 {
    unsafe {
        let count = *(hdr.wrapping_add(0x560) as *const u32);
        if count == 0 {
            return 1;
        }
        let mut ent = *(hdr.wrapping_add(0x564) as *const u32);
        let x = *(pt as *const f32);
        let y = *(pt.wrapping_add(4) as *const f32);
        let z = *(pt.wrapping_add(8) as *const f32);
        let abs_mask = *lf_rs89_rt::global::<u32>(0xFE8F80);
        let tol = *lf_rs89_rt::global::<f32>(0xFE8A94);
        let mut k: u32 = 0;
        while k < count {
            let r = *(ent.wrapping_add(0x10) as *const f32);
            let dx = x - *(ent as *const f32);
            let dy = y - *(ent.wrapping_add(4) as *const f32);
            if r * r > dx * dx + dy * dy {
                let dz = z - *(ent.wrapping_add(8) as *const f32);
                let adz = f32::from_bits(dz.to_bits() & abs_mask);
                if tol > adz {
                    return 0;
                }
            }
            k += 1;
            ent = ent.wrapping_add(0x20);
        }
        1
    }
});
