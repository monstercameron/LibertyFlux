// original: 0x00b38430 pool_collect_farthest_capped (proposed)

/// Collect up to a capped number of pool members with the largest squared
/// distance to a target point, then report each one.
///
/// `target` points to three floats (x, y, z). A direct thiscall against the
/// pool header (read from a global: item array at `+0x00`, flag bytes at
/// `+0x04`, item count at `+0x08`, item stride at `+0x0c`) is asked twice; if
/// the first answer reaches 12 the function does nothing. Otherwise the
/// capacity is `12 - answer`, clamped to 20 from above with an unsigned
/// comparison.
///
/// The pool is then scanned from the last index down to zero. Members whose
/// flag byte has bit 0x80 set are skipped, as are null slots and members
/// whose link at `+0x6c` is non-null with the byte at link `+0x0e` set. For
/// each accepted member the squared distance from the target to the member's
/// position (three floats at `+0x30` of the object at `+0x20`) is formed as
/// `(dy*dy + dx*dx) + dz*dz` with `d = target - position`. While fewer than
/// capacity members are held, the member is stored and the running maximum
/// tracks the largest distance seen (ordered-greater only, from +0.0). Once
/// full, a member is kept only when its distance is ordered-greater than the
/// maximum; it then replaces the slot holding the smallest distance (ties
/// keep the earliest slot, and a distance that never compares below the
/// 100000.0 seed replaces slot zero), and the maximum becomes the new
/// distance.
///
/// Finally every held member is reported in slot order through a cdecl call
/// with `(member, 1)`. The function ends with the CRT security-cookie check,
/// which preserves all registers.
///
/// Original: 0x00b38430 (cdecl, one stack word, no return value).
lf_checker_rt::export!(cdecl, rw_00b38430(target: u32) -> u32 {
    unsafe {
        const POOL_GLOBAL: u32 = 0x018b6f10;
        const COOKIE_GLOBAL: u32 = 0x01057fb4;
        const COOKIE_CHECK: u32 = 3;
        const MIN_SEED: u32 = 0x00e88900;
        const POOL_ITEMS: u32 = 0x00;
        const POOL_FLAGS: u32 = 0x04;
        const POOL_COUNT: u32 = 0x08;
        const POOL_STRIDE: u32 = 0x0c;
        const SKIP_BIT: u8 = 0x80;
        const MEMBER_POS: u32 = 0x20;
        const MEMBER_LINK: u32 = 0x6c;
        const LINK_IDLE: u32 = 0x0e;
        const POS_X: u32 = 0x30;
        const COUNT_LIMIT: u32 = 12;
        const SLOT_MAX: u32 = 20;
        const CALLEE_COUNT: u32 = 1;
        const CALLEE_REPORT: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        // Security cookie, computed against our own frame exactly the way the
        // original computes it against its: the value handed to the check is
        // the cookie global on both sides.
        let cookie = rd32(lf_checker_rt::relocated(COOKIE_GLOBAL));
        let slot: u32 = 0;
        let frame = core::ptr::addr_of!(slot) as u32;
        let saved = cookie ^ frame;

        let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
        let first = lf_checker_rt::callee_thiscall!(CALLEE_COUNT, u32, pool);
        if first >= COUNT_LIMIT {
            lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, saved ^ frame);
            return 0;
        }
        let second = lf_checker_rt::callee_thiscall!(CALLEE_COUNT, u32, pool);
        let mut cap = COUNT_LIMIT.wrapping_sub(second);
        if cap > SLOT_MAX {
            cap = SLOT_MAX;
        }
        let mut slots = [(0.0f32, 0u32); SLOT_MAX as usize];
        let mut held = 0u32;
        let mut best = 0.0f32;
        let count = rd32(pool.wrapping_add(POOL_COUNT));
        if count != 0 {
            let bits = rd32(pool.wrapping_add(POOL_FLAGS));
            let base = rd32(pool.wrapping_add(POOL_ITEMS));
            let estride = rd32(pool.wrapping_add(POOL_STRIDE));
            let seed = f32::from_bits(rd32(lf_checker_rt::relocated(MIN_SEED)));
            let mut idx = count;
            loop {
                idx = idx.wrapping_sub(1);
                if rd8(bits.wrapping_add(idx)) & SKIP_BIT == 0 {
                    let member = base.wrapping_add(estride.wrapping_mul(idx));
                    if member != 0 {
                        let link = rd32(member.wrapping_add(MEMBER_LINK));
                        if link == 0 || rd8(link.wrapping_add(LINK_IDLE)) == 0 {
                            let pos = rd32(member.wrapping_add(MEMBER_POS));
                            let dx = sub(
                                f32::from_bits(rd32(target)),
                                f32::from_bits(rd32(pos.wrapping_add(POS_X))),
                            );
                            let dy = sub(
                                f32::from_bits(rd32(target.wrapping_add(4))),
                                f32::from_bits(rd32(pos.wrapping_add(POS_X + 4))),
                            );
                            let dz = sub(
                                f32::from_bits(rd32(target.wrapping_add(8))),
                                f32::from_bits(rd32(pos.wrapping_add(POS_X + 8))),
                            );
                            let dist2 =
                                add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                            if held < cap {
                                slots[held as usize] = (dist2, member);
                                held += 1;
                                if dist2 > best {
                                    best = dist2;
                                }
                            } else if dist2 > best {
                                // First strictly-smallest slot wins; a value
                                // that never compares below the seed keeps
                                // slot zero. NaN slots never replace.
                                let mut m = seed;
                                let mut mi = 0u32;
                                let mut k = 0u32;
                                while k < held {
                                    let s = slots[k as usize].0;
                                    if s < m {
                                        m = s;
                                        mi = k;
                                    }
                                    k += 1;
                                }
                                slots[mi as usize] = (dist2, member);
                                best = dist2;
                            }
                        }
                    }
                }
                if idx == 0 {
                    break;
                }
            }
        }
        let mut i = 0u32;
        while i < held {
            lf_checker_rt::callee_cdecl!(CALLEE_REPORT, u32, slots[i as usize].1, 1);
            i += 1;
        }
        lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, saved ^ frame);
        0
    }
});
