// original: 0x00925D40 pick_slot_by_id_or_distance (proposed)

/// Pick an input slot: the highest-id empty slot, else the nearest past threshold.
///
/// Calls the readiness helper (callee 1); when it answers 0 there is nothing
/// to pick and the result is -1. Otherwise scans the five slot records from
/// `TABLE` to `TABLE_END` (stride `STRIDE`), each with an id at `+0x00` and
/// a state at `+0x08`:
/// - an empty slot (state -1) is a candidate by id: the first one is taken
///   unconditionally, later ones only when their id is *signed*-greater than
///   the best so far;
/// - an occupied slot contributes its distance float at `-0x0c`: when that
///   value is ordered-greater than the running threshold (starting at the
///   `limit` argument, updated to each new best), the slot becomes the
///   distance pick.
///
/// The id pick wins when one exists, else the distance pick, else -1. Float
/// compares are the original's `comiss` order, pinned with `black_box`.
///
/// Original: 0x00925D40 (cdecl, one float stack word). One direct call.
lf_checker_rt::export!(cdecl, rw_00925D40(limit: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x011A_0BEC;
        const TABLE_END: u32 = 0x011A_13EC;
        const STRIDE: u32 = 0x100;
        #[inline(always)]
        fn above(x: f32, y: f32) -> bool {
            core::hint::black_box(x) > core::hint::black_box(y)
        }
        let ready: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        if (ready as u8) == 0 {
            return 0xFFFF_FFFF;
        }
        let mut threshold = f32::from_bits(limit);
        let mut best_id: i32 = 0;
        let mut id_pick: i32 = -1;
        let mut dist_pick: i32 = -1;
        let mut slot = lf_checker_rt::relocated(TABLE);
        let end = lf_checker_rt::relocated(TABLE_END);
        let mut index: i32 = 0;
        while slot < end {
            let state = (slot.wrapping_add(8) as *const i32).read_unaligned();
            if state == -1 {
                let id = (slot as *const i32).read_unaligned();
                if id_pick == -1 || id > best_id {
                    best_id = id;
                    id_pick = index;
                }
            } else {
                let dist = f32::from_bits(
                    (slot.wrapping_sub(0x0c) as *const u32).read_unaligned(),
                );
                if above(dist, threshold) {
                    threshold = dist;
                    dist_pick = index;
                }
            }
            slot = slot.wrapping_add(STRIDE);
            index += 1;
        }
        if id_pick != -1 {
            id_pick as u32
        } else if dist_pick != -1 {
            dist_pick as u32
        } else {
            0xFFFF_FFFF
        }
    }
});
