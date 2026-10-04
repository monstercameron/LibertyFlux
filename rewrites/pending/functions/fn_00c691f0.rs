// original: 0x00c691f0 seeded_flag_lookup
// Look up a flag byte for a table id: the seed pointer must be set and the
// id's object must carry a non-negative index at +0xAC; the byte at the
// seeded base plus that index decides. Returns 1 for nonzero, else 0.
export!(stdcall, rw_00c691f0(id: u32) -> u32 {
    unsafe {
        let seed = *global::<u32>(0x169e3dc);
        if seed == 0 {
            return 0;
        }
        let ent = *(relocated(0x1295cd8).wrapping_add(id.wrapping_mul(4)) as *const u32);
        let s = *((ent as *const u8).add(0xac) as *const i32);
        if s < 0 {
            return 0;
        }
        let a = *global::<u32>(0x169c47c);
        let b = *global::<u32>(0x169c478);
        let t = (*((seed as *const u8).add(0x20)) & 0x7f) as u32;
        let off = a.wrapping_add(b.wrapping_mul(2)).wrapping_mul(0x47).wrapping_add(t).wrapping_mul(0x2a);
        let base = relocated(0x168ace8).wrapping_add(off);
        if *((base.wrapping_add(s as u32)) as *const u8) != 0 {
            1
        } else {
            0
        }
    }
});
