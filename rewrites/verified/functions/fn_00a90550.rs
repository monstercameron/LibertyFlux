// original: 0x00a90550 stream_dispatch_rows_slots

/// Dispatches row callbacks, then per-slot lookups with a filter.
///
/// Phase 1 walks `count` rows (`count` = u16 at `this+0xE8`, re-read each
/// step; row `i` at `[this+0xE4] + i * 0xA0`): each row's node chain (head
/// at row `+8`, next at node `+4`) yields callback values from `[sub+0x34]`
/// (`sub` = node's first word), and every non-zero value is invoked
/// (callee 1, cdecl) as `(value, a1, a2, 0, 1)`. Phase 2 asks callee 2
/// (thiscall) for a bound `n`, then for `outer` from `n - 1` down to 0 and
/// `k` in 0..4 calls callee 3 (thiscall) as `(0, outer, k)`; a live answer
/// `p` with `(p[0x28] & 0x3C0) == 0x100` and a non-zero byte at `p+0x22B`
/// is invoked like a callback with `(p, a1, a2, 0, 1)`. Returns the last
/// callee answer seen (`n` when phase 2 is skipped). Three callees.
/// Original: 0x00A90550 (thiscall, ECX + two stack words), 223 bytes.
lf_checker_rt::export!(thiscall, rw_00a90550(this: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const ROWS_OFF: u32 = 0xE4;
        const COUNT_OFF: u32 = 0xE8;
        const ROW_STRIDE: u32 = 0xA0;
        const HEAD_OFF: u32 = 8;
        const NEXT_OFF: u32 = 4;
        const FN_OFF: u32 = 0x34;
        const FILT_OFF: u32 = 0x28;
        const FILT_MASK: u32 = 0x3C0;
        const FILT_WANT: u32 = 0x100;
        const TAG_OFF: u32 = 0x22B;
        const INNER_N: u32 = 4;
        const INVOKE: u32 = 1;
        const BOUND: u32 = 2;
        const LOOKUP: u32 = 3;
        let mut count = (this.wrapping_add(COUNT_OFF) as *const u16).read_unaligned() as u32;
        if 0u32 < count {
            let rows = (this.wrapping_add(ROWS_OFF) as *const u32).read_unaligned();
            let mut i = 0u32;
            loop {
                let mut node = (rows
                    .wrapping_add(i.wrapping_mul(ROW_STRIDE))
                    .wrapping_add(HEAD_OFF) as *const u32)
                    .read_unaligned();
                while node != 0 {
                    let sub = (node as *const u32).read_unaligned();
                    node = (node.wrapping_add(NEXT_OFF) as *const u32).read_unaligned();
                    let f = (sub.wrapping_add(FN_OFF) as *const u32).read_unaligned();
                    if f != 0 {
                        let _: u32 = lf_checker_rt::callee_cdecl!(INVOKE, u32, f, a1, a2, 0, 1);
                    }
                }
                count =
                    (this.wrapping_add(COUNT_OFF) as *const u16).read_unaligned() as u32;
                i = i.wrapping_add(1);
                if i >= count {
                    break;
                }
            }
        }
        let n: u32 = lf_checker_rt::callee_thiscall!(BOUND, u32, this, 0);
        let mut last: u32 = n;
        let mut outer = (n as i32).wrapping_sub(1);
        if outer >= 0 {
            loop {
                let mut k = 0u32;
                while k < INNER_N {
                    let p: u32 =
                        lf_checker_rt::callee_thiscall!(LOOKUP, u32, this, 0, outer as u32, k);
                    last = p;
                    if p != 0 {
                        let w = (p.wrapping_add(FILT_OFF) as *const u32).read_unaligned();
                        if (w & FILT_MASK) == FILT_WANT {
                            let tag = (p.wrapping_add(TAG_OFF) as *const u8).read();
                            if tag != 0 {
                                let r: u32 =
                                    lf_checker_rt::callee_cdecl!(INVOKE, u32, p, a1, a2, 0, 1);
                                last = r;
                            }
                        }
                    }
                    k = k.wrapping_add(1);
                }
                outer = outer.wrapping_sub(1);
                if outer < 0 {
                    break;
                }
            }
        }
        last
    }
});
