// original: 0x00c56ca0 peds_pick_best_float_slot (proposed)

/// Pick the slot with the smallest float among nine, reporting its pointer.
///
/// `this` holds nine slots: floats at `+FLOAT0 + 4 * k`, candidate pointers
/// at `+PTR0 + 4 * k` and ids at `+ID0 + 4 * k` for `k` in 0..9. The two
/// out-words (`out_ptr`, `out_id`) start at zero. When the word at `+0x34`
/// is already set, slot 4 is taken at once (its id lives at `+0x34` and its
/// pointer at `+0x58`); otherwise the slots are scanned from a starting best
/// of the largest finite float and a slot replaces the best only when its
/// float is strictly below it and its pointer is non-zero, either of which
/// also refreshes the running best.
///
/// A float comparison that is unordered (a NaN on either side) keeps the
/// old best, exactly like the original's `comiss`/`jbe` pair, which Rust's
/// `>` reproduces: it is false for every unordered pair.
///
/// The original falls through with whatever `eax` last held, which is the
/// caller's entry value when no slot loads it, so the rewrite returns zero
/// and the return channel is not compared; the two out-words are the result.
///
/// Original: 0x00C56CA0 (thiscall, ECX = record, two stack words = out
/// pointer and out id, return channel ignored).
lf_checker_rt::export!(thiscall, rw_00c56ca0(this: u32, out_ptr: u32, out_id: u32) -> u32 {
    unsafe {
        const FAST_ID: u32 = 0x34;
        const FAST_PTR: u32 = 0x58;
        const FLOAT0: u32 = 0x6C;
        const PTR0: u32 = 0x48;
        const ID0: u32 = 0x24;
        const SLOTS: u32 = 9;
        const BEST_INIT: u32 = 0x00FE8D18;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(out_ptr, 0);
        wr32(out_id, 0);
        if rd32(this.wrapping_add(FAST_ID)) != 0 {
            wr32(out_ptr, rd32(this.wrapping_add(FAST_PTR)));
            wr32(out_id, rd32(this.wrapping_add(FAST_ID)));
            return 0;
        }
        let mut best = f32::from_bits(rd32(lf_checker_rt::relocated(BEST_INIT)));
        let mut k = 0u32;
        while k < SLOTS {
            let f = f32::from_bits(rd32(this.wrapping_add(FLOAT0).wrapping_add(k * 4)));
            if best > f {
                let p = rd32(this.wrapping_add(PTR0).wrapping_add(k * 4));
                if p != 0 {
                    wr32(out_ptr, p);
                    wr32(out_id, rd32(this.wrapping_add(ID0).wrapping_add(k * 4)));
                    best = f;
                }
            }
            k += 1;
        }
        0
    }
});
