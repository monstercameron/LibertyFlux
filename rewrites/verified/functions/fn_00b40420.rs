// original: 0x00b40420 box_zone_replace (proposed)

/// Replace the zone entries contained in a box, then append the box.
///
/// Holds two locks (declared stdcall in the contract: only the pushed
/// global addresses are compared, since the lock objects themselves are
/// stack temporaries whose addresses are not behaviour) while it works on
/// two global tables. First, every set, non-null row of the 12-byte notify
/// table is offered the box through a thiscall notify callee. Then every
/// 0x30-byte zone entry whose low corner is at or above `box_lo` and whose
/// high corner is at or below `box_hi` (componentwise, ordered float
/// comparison) is removed by shifting the later entries down; the shift
/// copies ten words and merges only bit 0 of the ninth, leaving the last
/// two words of each shifted entry untouched, exactly like the original.
/// Finally the box is appended as a new entry with the low byte of `flags`
/// merged into bit 0 of its ninth word and `tag` in its tenth, and both
/// locks are released. No return value.
///
/// Original: 0x00B40420 (cdecl, four stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b40420(box_lo: u32, box_hi: u32, flags: u32, tag: u32) -> u32 {
    unsafe {
        const LOCK_A: u32 = 0x016B8FA4;
        const LOCK_B: u32 = 0x016C84D0;
        const COUNT_ADDR: u32 = 0x016B6B38;
        const TABLE_ADDR: u32 = 0x016B8F84;
        const ZONE_COUNT: u32 = 0x016B6AFC;
        const ZONES: u32 = 0x016B9030;
        const ENTRY: u32 = 0x30;
        const CAL_LOCK_A: u32 = 2;
        const CAL_LOCK_B: u32 = 3;
        const CAL_NOTIFY: u32 = 4;
        const CAL_UNLOCK_A: u32 = 5;
        const CAL_UNLOCK_B: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn grdf(va: u32) -> f32 {
            unsafe { f32::from_bits(lf_checker_rt::global::<u32>(va).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn grdw(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn gw32(va: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(va).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd32_g(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32_g(va: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(va).write_unaligned(v) }
        }

        let _: u32 = lf_checker_rt::callee_stdcall!(
            CAL_LOCK_A, u32, lf_checker_rt::relocated(LOCK_A)
        );
        let _: u32 = lf_checker_rt::callee_stdcall!(
            CAL_LOCK_B, u32, lf_checker_rt::relocated(LOCK_B)
        );
        let n = grdw(COUNT_ADDR);
        let tab = grdw(TABLE_ADDR);
        let mut i = 0u32;
        while i < n {
            // Both the count and the table pointer are reloaded every row.
            let n_now = grdw(COUNT_ADDR);
            if i >= n_now {
                break;
            }
            let row = grdw(TABLE_ADDR).wrapping_add(i.wrapping_mul(12));
            if rd8(row) & 1 != 0 {
                let target = rd32(row.wrapping_add(4));
                if target != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_NOTIFY, u32, target, box_lo, box_hi, flags
                    );
                }
            }
            i = i.wrapping_add(1);
            let _ = (n, tab);
        }
        let mut m = grdw(ZONE_COUNT) as i32;
        if m > 0 {
            let mut j = 0i32;
            while j < m {
                let base = (ZONES as u64 + j as u64 * ENTRY as u64) as u32;
                let mut contained = true;
                for k in 0..3u32 {
                    if !(grdf(base.wrapping_add(k * 4)) >= rdf(box_lo.wrapping_add(k * 4))) {
                        contained = false;
                        break;
                    }
                }
                if contained {
                    for k in 0..3u32 {
                        if !(rdf(box_hi.wrapping_add(k * 4)) >= grdf(base.wrapping_add(0x10 + k * 4))) {
                            contained = false;
                            break;
                        }
                    }
                }
                if contained {
                    m -= 1;
                    let mut k = j;
                    while k < m {
                        let dst = (ZONES as u64 + k as u64 * ENTRY as u64) as u32;
                        let src = dst.wrapping_add(ENTRY);
                        for w in 0..8u32 {
                            wr32_g(dst.wrapping_add(w * 4), rd32_g(src.wrapping_add(w * 4)));
                        }
                        let old = rd32_g(dst.wrapping_add(0x20));
                        let new = rd32_g(src.wrapping_add(0x20));
                        wr32_g(dst.wrapping_add(0x20), old ^ ((new ^ old) & 1));
                        wr32_g(dst.wrapping_add(0x24), rd32_g(src.wrapping_add(0x24)));
                        k += 1;
                    }
                    gw32(ZONE_COUNT, m as u32);
                } else {
                    j += 1;
                }
            }
        }
        let m_now = grdw(ZONE_COUNT);
        let slot = (ZONES as u64 + m_now as u64 * ENTRY as u64) as u32;
        for w in 0..4u32 {
            wr32_g(slot.wrapping_add(w * 4), rd32(box_lo.wrapping_add(w * 4)));
        }
        for w in 0..4u32 {
            wr32_g(slot.wrapping_add(0x10 + w * 4), rd32(box_hi.wrapping_add(w * 4)));
        }
        let old = rd32_g(slot.wrapping_add(0x20));
        wr32_g(slot.wrapping_add(0x20), old ^ (((flags & 0xff) ^ old) & 1));
        wr32_g(slot.wrapping_add(0x24), tag);
        gw32(ZONE_COUNT, m_now.wrapping_add(1));
        let _: u32 = lf_checker_rt::callee_stdcall!(CAL_UNLOCK_A, u32,);
        let _: u32 = lf_checker_rt::callee_stdcall!(CAL_UNLOCK_B, u32,);
        0
    }
});
