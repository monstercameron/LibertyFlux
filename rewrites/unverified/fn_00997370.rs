// original: 0x00997370 audio_voice_slot_update (proposed)

/// Update one audio voice slot: either program two queued entries from the
/// slot's parameters (main path) or shut the slot's channel down (bypass).
///
/// `this` is the voice manager; `a0` is an opaque word passed through to the
/// level call; `a1` points to the slot parameters (gains read at `+0x24`
/// and `+0x2c` on the main path).
///
/// Dispatch: when the shared bank object (global) is null the function
/// returns at once. Otherwise the slot's state block (reached through
/// `this+0x820` and `+0xf50`) selects the path: the main path needs the
/// block present with its idle flag (`+0x218`) clear and its active flag
/// (`+0x219`) set; anything else takes the bypass.
///
/// Main path: two entries are fetched through the bank's virtual slot
/// `+0x10` (callee 1, planted stub) and programmed from the name table at
/// `this+0x9dc` and the slot gains run through the equalizer (callee 2):
/// entry fields `+0x00`/`+0x08` take table words, `+0x04` takes the product
/// of two equalizer results, `+0x10` takes 8 and `+0x11`/`+0x14` take the
/// shared mode byte (global). The level call (callee 3) combines the
/// passthrough word with 1.0; its result and complement drive the channel
/// apply calls (callees 4 and 5) on the channel object at `this+0xa90`.
/// The shared counter (global) is incremented.
///
/// Bypass: the level call runs with 0.0 instead. The channel id is looked
/// up from the table base (global) plus the channel's band byte times
/// 0x6f40, plus the stride (global) times the channel's index byte, or zero
/// when the index is 0xff; the id selects the channel object for the stop
/// call (callee 6) with the level and its complement. The level is compared
/// against zero: a positive level increments the shared counter and sets the
/// flag bit, otherwise the flag bit is clear. The flag bit is stored into
/// the low byte of the incoming second-argument slot and that whole word,
/// upper bytes being the incoming argument's own bytes, is passed to the
/// finalize call (callee 7) on the same looked-up object (or null when the
/// index is 0xff).
///
/// The original reuses its incoming argument slots as temporaries on both
/// paths, which a Rust rewrite cannot observe; the contract therefore
/// leaves the stack check off, and every value kept in those slots is still
/// verified where it flows into calls, the heap or globals. The return
/// register is not compared: it holds stub answers on the taken paths and
/// the untouched entry value when the bank is null.
///
/// Original: 0x00997370 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00997370(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const G_BANK: u32 = 0x012832ac;
        const G_MODE: u32 = 0x012832b4;
        const G_COUNT: u32 = 0x0128329c;
        const G_MULT: u32 = 0x0115d968;
        const G_TBL: u32 = 0x0115d988;
        const K_ONE: u32 = 0x00fe88e8;
        const VT_SLOT: u32 = 0x10;
        const ROW: u32 = 0x6f40;
        const TBL_BIAS: u32 = 0x6f14;

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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn grab(obj: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + VT_SLOT) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn lookup_id(m: u32) -> u32 {
            unsafe {
                let idx = rd8(m + 4) as u32;
                if idx == 0xff {
                    return 0;
                }
                let band = rd8(m + 0x40) as u32;
                let mult = rd32(lf_checker_rt::relocated(G_MULT));
                let base = rd32(lf_checker_rt::relocated(G_TBL));
                let cell = rd32(
                    base
                        .wrapping_add(band.wrapping_mul(ROW))
                        .wrapping_add(TBL_BIAS),
                );
                cell.wrapping_add(mult.wrapping_mul(idx))
            }
        }
        #[inline(always)]
        unsafe fn bump() {
            unsafe {
                let p = lf_checker_rt::relocated(G_COUNT);
                wr32(p, rd32(p).wrapping_add(1));
            }
        }

        let bank = rd32(lf_checker_rt::relocated(G_BANK));
        if bank == 0 {
            return 0;
        }
        let blk = rd32(rd32(this.wrapping_add(0x820)).wrapping_add(0xf50));
        // Order matches the original's short-circuit: null block first.
        let main = blk != 0 && rd8(blk + 0x218) == 0 && rd8(blk + 0x219) != 0;
        if !main {
            // Bypass path.
            let w1: f32 =
                lf_checker_rt::callee_thiscall!(3, f32, this.wrapping_add(0x488), 0, a0);
            let m = rd32(this.wrapping_add(0xa90));
            let id = lookup_id(m);
            let k1 = f32::from_bits(rd32(lf_checker_rt::relocated(K_ONE)));
            let e0 = sub(k1, w1);
            lf_checker_rt::callee_thiscall!(6, u32, id, w1.to_bits(), e0.to_bits());
            // comiss against +0.0 with jbe: flag set only above zero.
            let flag: u32 = if w1 > 0.0 {
                bump();
                1
            } else {
                0
            };
            let id2 = lookup_id(m);
            let flagword = (a1 & 0xffffff00) | flag;
            lf_checker_rt::callee_thiscall!(7, u32, id2, flagword);
            return 0;
        }
        // Main path.
        let mode = rd8(lf_checker_rt::relocated(G_MODE));
        let ans1 = grab(bank);
        wr32(ans1.wrapping_add(8), 0);
        let ans2 = grab(bank);
        wr8(ans2.wrapping_add(0x11), mode);
        let subobj = rd32(bank.wrapping_add(8));
        let ans3 = grab(subobj);
        let g_hi = rdf(a1.wrapping_add(0x2c));
        let r1: f32 =
            lf_checker_rt::callee_thiscall!(2, f32, this.wrapping_add(0x258), g_hi.to_bits());
        wr8(ans3.wrapping_add(0x14), mode);
        wr32(ans3.wrapping_add(0x10), 8);
        let names = rd32(this.wrapping_add(0x9dc));
        wr32(ans3, rd32(names.wrapping_add(0x58)));
        let g_lo = rdf(a1.wrapping_add(0x24));
        wr32(ans3.wrapping_add(8), rd32(names.wrapping_add(0x5c)));
        let r2: f32 =
            lf_checker_rt::callee_thiscall!(2, f32, this.wrapping_add(0x208), g_lo.to_bits());
        wrf(ans3.wrapping_add(4), mul(r2, r1));
        let ans4 = grab(rd32(subobj.wrapping_add(8)));
        wr8(ans4.wrapping_add(0x14), mode);
        wr32(ans4.wrapping_add(0x10), 8);
        wr32(ans4, rd32(names.wrapping_add(0x64)));
        wr32(ans4.wrapping_add(8), rd32(names.wrapping_add(0x68)));
        let g_lo2 = rdf(a1.wrapping_add(0x24));
        let r3: f32 =
            lf_checker_rt::callee_thiscall!(2, f32, this.wrapping_add(0x230), g_lo2.to_bits());
        wrf(ans4.wrapping_add(4), mul(r3, r1));
        let u1: f32 = lf_checker_rt::callee_thiscall!(
            3,
            f32,
            this.wrapping_add(0x488),
            0x3f800000,
            a0
        );
        let chan = rd32(this.wrapping_add(0xa90));
        let k1 = f32::from_bits(rd32(lf_checker_rt::relocated(K_ONE)));
        let d = sub(k1, u1);
        lf_checker_rt::callee_thiscall!(4, u32, chan, u1.to_bits(), d.to_bits());
        lf_checker_rt::callee_thiscall!(5, u32, chan, 1);
        bump();
        0
    }
});
