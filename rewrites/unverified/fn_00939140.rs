// original: 0x00939140 stream_best_score_select (proposed)

/// Pick the highest streaming score and return its record's tag.
///
/// Reads up to `count` scores (the counter call, asked again each round)
/// from the score table; the winner is the first entry strictly greater
/// than everything before it (a NaN never wins, matching the hardware
/// compare-then-jump-below shape). Fetches the winner's object and returns
/// the dword at slot `+0x0A`, or 0 when any link is null.
lf_checker_rt::export!(cdecl, rw_00939140() -> u32 {
    unsafe {
        const COUNT: u32 = 1;
        const FETCH: u32 = 2;
        const SCORES: u32 = 0x11A4F20;
        const SLOT_LINK: u32 = 0x17D0;
        const TAG_FIELD: u32 = 0x0A;
        let base = lf_checker_rt::relocated(SCORES);
        let n: u32 = lf_checker_rt::callee_cdecl!(COUNT, u32,);
        let mut best = f32::from_bits((base as *const u32).read_unaligned());
        let mut picked: u8 = 0;
        let mut next: u8 = 1;
        if n > 1 {
            let mut i: u32 = 1;
            loop {
                let f = f32::from_bits(
                    ((base + i * 4) as *const u32).read_unaligned());
                // Ordered greater only: NaN on either side keeps the old best.
                if f > best {
                    picked = next;
                    best = f;
                }
                next = next.wrapping_add(1);
                i = next as u32;
                let m: u32 = lf_checker_rt::callee_cdecl!(COUNT, u32,);
                if i >= m {
                    break;
                }
            }
        }
        let obj: u32 = lf_checker_rt::callee_cdecl!(FETCH, u32, picked as u32);
        if obj == 0 {
            0
        } else {
            let slot = ((obj + SLOT_LINK) as *const u32).read_unaligned();
            if slot == 0 {
                0
            } else {
                ((slot + TAG_FIELD) as *const u32).read_unaligned()
            }
        }
    }
});
