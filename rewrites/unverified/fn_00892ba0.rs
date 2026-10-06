// original: 0x00892BA0 audsound_build_voice_entries
/// Builds one voice entry per pool record from the selected source bank.
///
/// The dword at `this+0x5c` selects the source holder (`this+0x80` for 0,
/// `+0x84` for 1, ...); the argument's low byte (`flag`) gates the tail add.
/// `p = [this+0x7c]` is the pool header
/// with `n = [p+0x24]` records; returns `p` at once when `n` is zero. For
/// record `i` (entry pointer `e = [[p+0x14] + i*16]`, source row
/// `s = [holder] + i*16`): stores `(v0 << 11) + holder + [this+0x60]` at `[e]`
/// where `v0 = [s]`, and -1 at `[e+0x14]`; then with `v1 = [s+0xc]` and
/// `v2 = [s+8]`: when `v1` is zero stores 0 at `[e+0xc]` and, only when `v2`
/// is also nonzero, adds `v2 + v2` to `[e]`; otherwise stores the SIGNED half
/// `v1 / 2` at `[e+0xc]`, and when `flag` is nonzero and `v2` is nonzero
/// stores `v2` at `[e+0x28]` and adds the unsigned half `v2 / 2` to `[e]`;
/// when `flag` is zero or `v2` is zero stores 0 at `[e+0x28]` instead. The
/// zero-`v1` path never touches `[e+0x28]` and ignores `flag`. All counts and
/// the `v2` halving are unsigned; only the `v1` halving is signed
/// (cdq/sub/sar). Returns `p`.
/// Original: 0x00892BA0 (thiscall, one stack word: flag).
export!(thiscall, rw_00892BA0(this: *mut u8, flag: u32) -> u32 {
    unsafe {
        const POOL: usize = 0x7c;
        const COUNT: usize = 0x24;
        const RECS: usize = 0x14;
        const HOLDERS: usize = 0x80;
        const BIAS: usize = 0x60;
        const REC_STRIDE: u32 = 16;
        const SHIFT: u32 = 11;
        let p = *(this.add(POOL) as *const u32);
        let n = ((p.wrapping_add(COUNT as u32)) as *const u32).read_unaligned();
        if n == 0 {
            return p;
        }
        let recs = ((p.wrapping_add(RECS as u32)) as *const u32).read_unaligned();
        let bank = *(this.add(0x5c) as *const u32);
        let holder = *(this.add(HOLDERS).add((bank as usize).wrapping_mul(4)) as *const u32);
        let src = (holder as *const u32).read_unaligned();
        let bias = *(this.add(BIAS) as *const u32);
        let mut i = 0u32;
        while i < n {
            let e = ((recs.wrapping_add(i.wrapping_mul(REC_STRIDE))) as *const u32).read_unaligned();
            let s = src.wrapping_add(i.wrapping_mul(REC_STRIDE));
            let v0 = (s as *const u32).read_unaligned();
            let head = v0.wrapping_shl(SHIFT).wrapping_add(holder).wrapping_add(bias);
            (e as *mut u32).write_unaligned(head);
            ((e.wrapping_add(0x14)) as *mut u32).write_unaligned(0xffff_ffff);
            let v1 = ((s.wrapping_add(0xc)) as *const u32).read_unaligned();
            let v2 = ((s.wrapping_add(8)) as *const u32).read_unaligned();
            if v1 == 0 {
                ((e.wrapping_add(0xc)) as *mut u32).write_unaligned(0);
                if v2 != 0 {
                    let h = (e as *const u32).read_unaligned();
                    (e as *mut u32).write_unaligned(h.wrapping_add(v2.wrapping_add(v2)));
                }
            } else {
                // Signed half: trunc(v1 / 2), as cdq/sub/sar computes it.
                let half = (v1 as i32).wrapping_div(2) as u32;
                ((e.wrapping_add(0xc)) as *mut u32).write_unaligned(half);
                // The original tests the flag byte here but consumes the
                // flags at the v2 test below; the effect is flag-gated.
                if (flag as u8) != 0 && v2 != 0 {
                    ((e.wrapping_add(0x28)) as *mut u32).write_unaligned(v2);
                    let h = (e as *const u32).read_unaligned();
                    (e as *mut u32).write_unaligned(h.wrapping_add(v2 >> 1));
                } else {
                    ((e.wrapping_add(0x28)) as *mut u32).write_unaligned(0);
                }
            }
            i = i.wrapping_add(1);
        }
        p
    }
});
