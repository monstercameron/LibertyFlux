// original: 0x00AD4A70 audio_register_pair_bounded (proposed)

/// Register a bounded audio channel pair.
///
/// When both arguments are in [0, 12), the pair is reported first
/// (cdecl/3 with a zero tag). When both are in [1, 10] there is nothing
/// more to do. Otherwise the pair is appended to the registry: unless the
/// entry count has reached 0x190, a use counter is bumped, the two
/// arguments are stored as half-words into the per-pair slots, and the
/// count is incremented. Cdecl/2; returns the last helper answer.
lf_checker_rt::export!(cdecl, rw_00ad4a70(a0: u32, a1: u32) -> u32 {
    unsafe {
        const REPORT: u32 = 1;
        const REGISTRY: u32 = 2;
        const COUNT_OFF: u32 = 0xE68;
        const SLOT_A_OFF: u32 = 0x820;
        const SLOT_B_OFF: u32 = 0xB40;
        const MAX_COUNT: u32 = 0x190;
        const USES: u32 = 0x01550DF0;
        const LIMIT: i32 = 12;
        const INNER: i32 = 11;
        let x = a0 as i32;
        let y = a1 as i32;
        let mut r: u32;
        if x >= 0 && y >= 0 && x < LIMIT && y < LIMIT {
            r = lf_checker_rt::callee_cdecl!(REPORT, u32, a0, a1, 0u32);
        } else {
            r = 0;
        }
        if x > 0 && y > 0 && x < INNER && y < INNER {
            return r;
        }
        let base = lf_checker_rt::callee_cdecl!(REGISTRY, u32,);
        r = base;
        let count = (base.wrapping_add(COUNT_OFF) as *const u32).read();
        if count >= MAX_COUNT {
            return r;
        }
        let uses = lf_checker_rt::global::<u32>(USES).read();
        (uses as *mut u32).write((uses as *const u32).read().wrapping_add(1));
        let base2 = lf_checker_rt::callee_cdecl!(REGISTRY, u32,);
        r = base2;
        let n = (base2.wrapping_add(COUNT_OFF) as *const u32).read();
        (base2.wrapping_add(SLOT_A_OFF).wrapping_add(n.wrapping_mul(2)) as *mut u16)
            .write(a0 as u16);
        let base3 = lf_checker_rt::callee_cdecl!(REGISTRY, u32,);
        r = base3;
        let m = (base3.wrapping_add(COUNT_OFF) as *const u32).read();
        (base3.wrapping_add(SLOT_B_OFF).wrapping_add(m.wrapping_mul(2)) as *mut u16)
            .write(a1 as u16);
        (base3.wrapping_add(COUNT_OFF) as *mut u32)
            .write((base3.wrapping_add(COUNT_OFF) as *const u32).read().wrapping_add(1));
        r
    }
});
