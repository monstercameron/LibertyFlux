// original: 0x00892AD0 audsound_relocate_pool_pointers
/// Rebases the pool's self-relative pointers by adding the pool base.
///
/// `this+0x7c` points at the pool header `p`. Returns at once when the enable
/// byte at `this+0x4d` is clear (returning entry EAX untouched, so the
/// contract pins entry EAX to 0 and the rewrite returns 0) or `p` is null
/// (returning 0). Otherwise adds `p` (as an
/// integer, wrapping) to the words at `p+0x14` and `p+0x1c`, then for each of
/// the `n = [p+0x24]` records (16 bytes each, starting at the rebased
/// `[p+0x14]`): adds `eb = (n << 4) + rebased [p+0x14]` to the record pointer
/// and follows it to `s`, adds the rebased `[p+0x2c]` to `[s]`, and when
/// `[s+0x1c]` has bit 0x400 set also adds `s` to `[s+0x20]`. Then for each of
/// the `m = [p+0x28]` records at the rebased `[p+0x1c]` adds
/// `(m << 4) + rebased [p+0x1c]` to the record word. Finally adds `p` to `[p]`.
/// All counts compare unsigned. Returns `p`.
/// Original: 0x00892AD0 (thiscall, no stack arguments).
export!(thiscall, rw_00892AD0(this: *mut u8) -> u32 {
    unsafe {
        const ENABLE: usize = 0x4d;
        const POOL: usize = 0x7c;
        const RECS_A: usize = 0x14;
        const RECS_B: usize = 0x1c;
        const COUNT_A: usize = 0x24;
        const COUNT_B: usize = 0x28;
        const AUX: usize = 0x2c;
        const REC_STRIDE: u32 = 16;
        const CHAIN_FLAG: u32 = 0x400;
        if *this.add(ENABLE) == 0 {
            return 0;
        }
        let p = *(this.add(POOL) as *const u32);
        if p == 0 {
            return 0;
        }
        let add_p = |at: u32| {
            let v = (at as *const u32).read_unaligned();
            (at as *mut u32).write_unaligned(v.wrapping_add(p));
        };
        add_p(p.wrapping_add(RECS_A as u32));
        add_p(p.wrapping_add(RECS_B as u32));
        let n = ((p.wrapping_add(COUNT_A as u32)) as *const u32).read_unaligned();
        let qa = ((p.wrapping_add(RECS_A as u32)) as *const u32).read_unaligned();
        let eb = n.wrapping_shl(4).wrapping_add(qa);
        let mut i = 0u32;
        while i < n {
            let slot = qa.wrapping_add(i.wrapping_mul(REC_STRIDE));
            let old = (slot as *const u32).read_unaligned();
            let s = old.wrapping_add(eb);
            (slot as *mut u32).write_unaligned(s);
            let aux = ((p.wrapping_add(AUX as u32)) as *const u32).read_unaligned();
            let head = (s as *const u32).read_unaligned();
            (s as *mut u32).write_unaligned(head.wrapping_add(aux.wrapping_add(p)));
            let flags = ((s.wrapping_add(0x1c)) as *const u32).read_unaligned();
            if flags & CHAIN_FLAG != 0 {
                let nxt = ((s.wrapping_add(0x20)) as *const u32).read_unaligned();
                ((s.wrapping_add(0x20)) as *mut u32).write_unaligned(nxt.wrapping_add(s));
            }
            i = i.wrapping_add(1);
        }
        let m = ((p.wrapping_add(COUNT_B as u32)) as *const u32).read_unaligned();
        let qb = ((p.wrapping_add(RECS_B as u32)) as *const u32).read_unaligned();
        let eb2 = m.wrapping_shl(4).wrapping_add(qb);
        let mut j = 0u32;
        while j < m {
            let slot = qb.wrapping_add(j.wrapping_mul(REC_STRIDE));
            let old = (slot as *const u32).read_unaligned();
            (slot as *mut u32).write_unaligned(old.wrapping_add(eb2));
            j = j.wrapping_add(1);
        }
        add_p(p);
        p
    }
});
