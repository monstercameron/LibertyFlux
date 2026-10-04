// original: 0x00d29870 CPedTargetting::vf4
/// Advance the round-robin cursor (`+0x234`, word): scan the next nine
/// entries for the first live slot (`[this+cand*64+0x34]` non-null, index
/// wrapped into 0..8), select it through virtual slot `+0xc` with
/// (`cand`, 0), store it and return the answer; when none is live store 0
/// and return 0.
///
/// Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00d29870(this: u32) -> u32 {
    unsafe {
        const CURSOR: u32 = 0x234;
        const STRIDE: i32 = 64;
        const SLOT_OFF: i32 = 0x34;
        const VT_SELECT: u32 = 0x0c;
        let idx = (this + CURSOR) as *const i16;
        let idxv = idx.read_unaligned() as i32;
        for k in 1..10i32 {
            let mut cand = k + idxv;
            if cand >= 8 {
                cand = idxv - 8 + k;
            }
            let slot = (this as i32).wrapping_add(cand.wrapping_mul(STRIDE)).wrapping_add(SLOT_OFF);
            if (slot as *const u32).read_unaligned() != 0 {
                let vt = (this as *const u32).read_unaligned();
                let tgt = ((vt + VT_SELECT) as *const u32).read_unaligned();
                let sel: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                let r = sel(this, cand as u32, 0);
                ((this + CURSOR) as *mut u16).write_unaligned(cand as u16);
                return r;
            }
        }
        ((this + CURSOR) as *mut u16).write_unaligned(0);
        0
    }
});
