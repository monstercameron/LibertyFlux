// original: 0x00abc170 sift_up_hole

/// Sift a hole up a sentinel-terminated binary heap towards a limit index.
///
/// While the hole sits above the limit, the parent element moves down into
/// it unless the parent slot holds the empty-slot marker, which ends the
/// walk immediately. The carried value is then stored into the final hole.
/// Returns the carried value after a walk, or the base pointer when the hole
/// already sits at or below the limit.
export!(cdecl, rs64_abc170(base: *const u32, hole: u32, limit: u32, value: u32, _x: u32) -> u32 {
    unsafe {
        const EMPTY_FILEVA: u32 = 0x0151_0A90;
        let empty = relocated(EMPTY_FILEVA);
        if (hole as i32) > (limit as i32) {
            let mut h = hole;
            let mut p = (hole as i32).wrapping_sub(1) / 2;
            loop {
                let v = *base.add(p as usize);
                if v == empty {
                    break;
                }
                *(base.add(h as usize) as *mut u32) = v;
                h = p as u32;
                p = (p as i32).wrapping_sub(1) / 2;
                if !((h as i32) > (limit as i32)) {
                    break;
                }
            }
            *(base.add(h as usize) as *mut u32) = value;
            value
        } else {
            *(base.add(hole as usize) as *mut u32) = value;
            base as u32
        }
    }
});
