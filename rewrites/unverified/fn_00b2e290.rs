// original: 0x00b2e290 wreck_task_scan_nearest (proposed)

/// Scan the ped pool for the two nearest eligible entries to this task's
/// position, and fill the two result slots with them.
///
/// `this` is the task object (only passed to callees, never read here).
/// `slots` points to two 0x48-byte result records, both first passed to the
/// slot-initialiser callee. `notify_flag`'s low byte decides whether each
/// filled slot is also reported to the wreck manager through its virtual
/// slot at `+8`.
///
/// The pool header is read through a global (base at `+0`, flag bytes at
/// `+4`, entry count at `+8`, entry stride at `+0x0c`); entries are scanned
/// from the last to the first, skipping ones whose flag byte has bit 0x80
/// set. An entry is eligible when the range callee accepts its position
/// block (`+0x20`, offset `+0x30`), its mode byte at `+0x10b8` is not 2, the
/// busy-check callee answers false, its state word at `+0x1304` is 0, 1 or
/// 4, and the final filter callee answers true. Distance is the Euclidean
/// distance between the entry's position (three floats at `+0x30` past the
/// `+0x20` block) and this task's position (three floats written by the
/// position callee), compared against a maximum-distance constant read from
/// a global; the two smallest distances win, ties keeping the earlier scan
/// (later pool index). The float operation order is the original's.
///
/// Each winning entry fills the next result slot through the fill callee
/// unless it has a guard object (`+0x6c`) whose byte at `+0x0e` is nonzero;
/// result slots are compacted (a skipped winner does not consume a slot).
/// Slots left unfilled are passed to the slot-initialiser callee again.
///
/// The return value is the last callee answer, except when both slots were
/// filled, in which case the slot pointer has been advanced twice and the
/// value is `slots + 0x90` (the original falls through with the advanced
/// pointer in eax).
///
/// Original: 0x00b2e290 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00b2e290(this: u32, slots: u32, notify_flag: u32) -> u32 {
    unsafe {
        const POOL_GLOBAL: u32 = 0x12E22A4;
        const MAX_DIST_GLOBAL: u32 = 0xE9BD14;
        const MGR_GLOBAL: u32 = 0x166D9FC;
        const POOL_BASE: u32 = 0x00;
        const POOL_FLAGS: u32 = 0x04;
        const POOL_COUNT: u32 = 0x08;
        const POOL_STRIDE: u32 = 0x0c;
        const ENTRY_POS_BLOCK: u32 = 0x20;
        const ENTRY_GUARD: u32 = 0x6c;
        const ENTRY_MODE: u32 = 0x10b8;
        const ENTRY_STATE: u32 = 0x1304;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const POS_Z: u32 = 0x38;
        const GUARD_READY: u32 = 0x0e;
        const SLOT_STRIDE: u32 = 0x48;
        const FLAG_HIDDEN: u8 = 0x80;
        const MODE_EXCLUDED: u8 = 2;
        const VTABLE_SLOT_NOTIFY: u32 = 0x08;
        const C_POS: u32 = 1;
        const C_INIT: u32 = 2;
        const C_RANGE: u32 = 3;
        const C_BUSY: u32 = 4;
        const C_FILTER: u32 = 5;
        const C_FILL: u32 = 6;
        // contract id 7 (manager notify) is reached through the planted vtable.
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        // Own position, written by the position callee into scratch.
        let mut here = [0u32; 3];
        let mut tail: u32 =
            lf_checker_rt::callee_thiscall!(C_POS, u32, this, here.as_mut_ptr() as u32);
        let (hx, hy, hz) = (
            f32::from_bits(here[0]),
            f32::from_bits(here[1]),
            f32::from_bits(here[2]),
        );

        // Both result slots start initialised.
        tail = lf_checker_rt::callee_thiscall!(C_INIT, u32, slots);
        tail = lf_checker_rt::callee_thiscall!(C_INIT, u32, slots.wrapping_add(SLOT_STRIDE));

        // Scan the pool from the last entry to the first, keeping the two
        // smallest distances. `comiss a,b; jbe` is `!(a > b)`, unordered
        // (NaN) included, so a NaN distance loses to both slots.
        let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
        let max_dist = rdf(lf_checker_rt::relocated(MAX_DIST_GLOBAL));
        let mut best_dist = [max_dist, max_dist];
        let mut best_entry = [0u32, 0u32];
        let flags = rd32(pool.wrapping_add(POOL_FLAGS));
        let base = rd32(pool.wrapping_add(POOL_BASE));
        let stride = rd32(pool.wrapping_add(POOL_STRIDE));
        let mut i = rd32(pool.wrapping_add(POOL_COUNT));
        if i != 0 {
            loop {
                i = i.wrapping_sub(1);
                if rd8(flags.wrapping_add(i)) & FLAG_HIDDEN == 0 {
                    let entry = base.wrapping_add(stride.wrapping_mul(i));
                    if entry != 0 {
                        let block = rd32(entry.wrapping_add(ENTRY_POS_BLOCK));
                        let in_range: u32 = lf_checker_rt::callee_thiscall!(
                            C_RANGE,
                            u32,
                            this,
                            block.wrapping_add(POS_X)
                        );
                        tail = in_range;
                        if in_range as u8 != 0
                            && rd8(entry.wrapping_add(ENTRY_MODE)) != MODE_EXCLUDED
                        {
                            let busy: u32 =
                                lf_checker_rt::callee_thiscall!(C_BUSY, u32, entry);
                            tail = busy;
                            if busy as u8 == 0 {
                                let state = rd32(entry.wrapping_add(ENTRY_STATE));
                                if state == 0 || state == 1 || state == 4 {
                                    let ok: u32 =
                                        lf_checker_rt::callee_cdecl!(C_FILTER, u32, entry);
                                    tail = ok;
                                    if ok as u8 != 0 {
                                        let blk =
                                            rd32(entry.wrapping_add(ENTRY_POS_BLOCK));
                                        let dy = sub(rdf(blk.wrapping_add(POS_Y)), hy);
                                        let dx = sub(rdf(blk.wrapping_add(POS_X)), hx);
                                        let dz = sub(rdf(blk.wrapping_add(POS_Z)), hz);
                                        let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                                        let d = core::hint::black_box(d2).sqrt();
                                        if best_dist[0] > d {
                                            best_dist[1] = best_dist[0];
                                            best_entry[1] = best_entry[0];
                                            best_dist[0] = d;
                                            best_entry[0] = entry;
                                        } else if best_dist[1] > d {
                                            best_dist[1] = d;
                                            best_entry[1] = entry;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if i == 0 {
                    break;
                }
            }
        }

        // Fill one result slot per winner, compacting past skipped ones.
        let mut filled = 0u32;
        let mut slot = slots;
        for k in 0..2u32 {
            let cand = best_entry[k as usize];
            if cand != 0 {
                let guard = rd32(cand.wrapping_add(ENTRY_GUARD));
                if guard == 0 || rd8(guard.wrapping_add(GUARD_READY)) == 0 {
                    tail = lf_checker_rt::callee_thiscall!(C_FILL, u32, slot, cand);
                    if notify_flag as u8 != 0 {
                        let mgr = rd32(lf_checker_rt::relocated(MGR_GLOBAL));
                        let notify: extern "thiscall" fn(u32, u32) -> u32 =
                            core::mem::transmute(
                                rd32(rd32(mgr).wrapping_add(VTABLE_SLOT_NOTIFY)) as usize,
                            );
                        tail = notify(mgr, cand);
                    }
                    filled += 1;
                    slot = slot.wrapping_add(SLOT_STRIDE);
                }
            }
        }

        // Slots left unfilled are initialised again; with both filled the
        // original returns the twice-advanced slot pointer.
        if filled < 2 {
            let mut s = slots.wrapping_add(filled.wrapping_mul(SLOT_STRIDE));
            let mut n = 2 - filled;
            while n != 0 {
                tail = lf_checker_rt::callee_thiscall!(C_INIT, u32, s);
                s = s.wrapping_add(SLOT_STRIDE);
                n -= 1;
            }
            tail
        } else {
            slot
        }
    }
});
