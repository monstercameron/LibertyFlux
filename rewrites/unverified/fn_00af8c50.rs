// original: 0x00AF8C50 veh_best_zone_pick (proposed)

/// Pick the cheapest zone slot containing a point.
///
/// `point` is tested against the zone slots starting at `FIRST`, each 40
/// bytes with its 9170-style bounds at slot + 0x10. Slot 0 is the default
/// answer with score `w(+0x16) - w(+0x10) - w(+0x12) + w(+0x18)` over its
/// signed bound words. Each further slot (while the global slot count
/// exceeds 1) is tested with the box predicate (callee 1); a containing
/// slot whose score `w(-2) - w(-8) - w(-6) + w(0)` above its cursor is
/// unsigned below the best score so far takes the lead. Returns the winning
/// slot's address.
///
/// Original: 0x00AF8C50 (cdecl, one stack argument, address in EAX).
lf_checker_rt::export!(cdecl, rw_00AF8C50(point: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 0x15FCCE0;
        const REST: u32 = 0x15FCD20;
        const COUNT: u32 = 0x15FFBC0;
        const SLOT: u32 = 0x28;
        const BOX_TEST: u32 = 1;
        #[inline(always)]
        unsafe fn w(p: u32) -> i32 {
            unsafe { (p as *const i16).read_unaligned() as i32 }
        }
        let first = lf_checker_rt::relocated(FIRST);
        let mut best = first;
        let mut edge = w(first + 0x16).wrapping_sub(w(first + 0x10)).wrapping_sub(w(first + 0x12)).wrapping_add(w(first + 0x18));
        let count = (lf_checker_rt::global::<u32>(COUNT) as *const u32).read_unaligned();
        if count > 1 {
            for k in 0..count.wrapping_sub(1) {
                let cur = lf_checker_rt::relocated(REST).wrapping_add(k.wrapping_mul(SLOT));
                let hit: u8 = lf_checker_rt::callee_cdecl!(BOX_TEST, u8, point, cur.wrapping_sub(0x18));
                if hit != 0 {
                    let s = w(cur.wrapping_sub(2)).wrapping_sub(w(cur.wrapping_sub(8))).wrapping_sub(w(cur.wrapping_sub(6))).wrapping_add(w(cur));
                    if (s as u32) < (edge as u32) {
                        edge = s;
                        best = cur.wrapping_sub(0x18);
                    }
                }
            }
        }
        best
    }
});
