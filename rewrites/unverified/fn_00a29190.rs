// original: 0x00a29190 task_state_gate (proposed)

/// Gate a ped task on its state flags, then either refresh a blend weight and
/// bump a retry counter or set/clear/toggle a status bit.
///
/// `this` points to the task; `a2` points to a large state block (or is null);
/// `a1`, `a4` and `a5` are used as zero/non-zero flags and `a3` is unread.
/// The sub-object at `this+SUB_OFF` gates everything: a null pointer or a
/// mode field (`+MODE_OFF` masked with `MODE_MASK`) other than `MODE_WANT
/// returns at once. Otherwise one table word selected by the counter at
/// `+COUNT_OFF` is passed to the id 1 callee (cdecl, result ignored), and
/// when the flag at `+BUSY_OFF` is set the id 2 callee (cdecl, always passed
/// 1) decides the `ok` bit, falling back to the id 3 callee (cdecl, no
/// arguments) when id 2 answers zero.
///
/// The state block is then classified by xors of byte pairs against 0x7f
/// into the refresh path or the gate path. The refresh path stops early
/// unless the second global byte is zero, then writes 1.0f to the weight
/// slot (`[this+WEIGHT_OFF]+WEIGHT_SLOT`) and, by the flags, increments one
/// saturating counter (`+CNT_A` past 1 wraps to 0, `+CNT_B`/`+CNT_C` past 2
/// wrap to 0, the third only when the busy flag read earlier was clear). The
/// gate path clears the status bit at `+STATUS_OFF` when `ok` is clear and
/// otherwise stops unless `a4` is zero, the third global byte is non-zero,
/// `a2` is non-null, and a float-table entry selected by the same counter
/// differs from the reference constant (an unordered comparison also
/// continues). It then sets the status bit when one byte-pair xor exceeds
/// 0x7f, clears it when a second pair does, and otherwise asks the id 4
/// callee (thiscall on `a2+ID4_BIAS`) and toggles the status bit on a
/// non-zero answer. A clear `ok` bit clears the status bit again at the end.
///
/// The function has no designed return value: the early exits return with
/// EAX untouched (whatever the caller had there), so the contract compares
/// everything except the return channel.
///
/// Original: 0x00a29190 (thiscall, five stack words; the third is unread).
lf_checker_rt::export!(thiscall, rw_00a29190(this: u32, a1: u32, a2: u32, _a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 0x0204;
        const MODE_OFF: u32 = 0x0028;
        const MODE_MASK: u32 = 0x03c0;
        const MODE_WANT: u32 = 0x00c0;
        const COUNT_OFF: u32 = 0x02b0;
        const BUSY_OFF: u32 = 0x0d68;
        const WEIGHT_OFF: u32 = 0x012c;
        const WEIGHT_SLOT: u32 = 0x1460;
        const ONE_BITS: u32 = 0x3f80_0000;
        const CNT_A: u32 = 0x0214;
        const CNT_B: u32 = 0x0215;
        const CNT_C: u32 = 0x0213;
        const STATUS_OFF: u32 = 0x0200;
        const XOR_LIMIT: u8 = 0x7f;
        const DL_OFF: u32 = 0x328d;
        const K1_OFF: u32 = 0x269c;
        const K2_OFF: u32 = 0x269e;
        const K3_OFF: u32 = 0x269f;
        const P1A_OFF: u32 = 0x2c6c;
        const P1B_OFF: u32 = 0x2c6e;
        const P2A_OFF: u32 = 0x2c7c;
        const P2B_OFF: u32 = 0x2c7e;
        const ID4_BIAS: u32 = 0x2708;
        const GLOBAL_A: u32 = 0x011f_701f;
        const GLOBAL_B: u32 = 0x0161_54b2;
        const GLOBAL_C: u32 = 0x0103_ce47;
        const FLOAT_TABLE: u32 = 0x00e9_b814;
        const FLOAT_REF: u32 = 0x00fe_8628;

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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn status_set(this: u32) {
            unsafe { wr8(this + STATUS_OFF, rd8(this + STATUS_OFF) | 1) }
        }
        #[inline(always)]
        unsafe fn status_clear(this: u32) {
            unsafe { wr8(this + STATUS_OFF, rd8(this + STATUS_OFF) & 0xfe) }
        }

        let sub = rd32(this.wrapping_add(SUB_OFF));
        if sub == 0 {
            return 0;
        }
        if rd32(sub.wrapping_add(MODE_OFF)) & MODE_MASK != MODE_WANT {
            return 0;
        }
        let count = rd32(sub.wrapping_add(COUNT_OFF));
        let sel = count.wrapping_add(3).wrapping_mul(3);
        let picked = rd32(sub.wrapping_add(sel.wrapping_mul(4)).wrapping_add(COUNT_OFF));
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, picked);
        let mut ok = true;
        let busy = rd32(sub.wrapping_add(BUSY_OFF)) != 0;
        if busy {
            let r2: u32 = lf_checker_rt::callee_cdecl!(2, u32, 1);
            if (r2 as u8) == 0 {
                let r3: u32 = lf_checker_rt::callee_cdecl!(3, u32);
                if (r3 as u8) == 0 {
                    ok = false;
                }
            }
        }
        // Classify the state block into the refresh path (true) or the
        // gate path (false), following the original's xor tests in order.
        let mut refresh = false;
        if a2 != 0 {
            let dl = rd8(a2.wrapping_add(DL_OFF));
            if dl != 0 {
                let k = rd8(a2.wrapping_add(K1_OFF));
                if rd8(a2.wrapping_add(K2_OFF)) ^ k > XOR_LIMIT
                    && rd8(a2.wrapping_add(K3_OFF)) ^ k <= XOR_LIMIT
                {
                    refresh = true;
                }
            } else {
                let k = rd8(a2.wrapping_add(K1_OFF));
                if rd8(a2.wrapping_add(K2_OFF)) ^ k <= XOR_LIMIT
                    && rd8(a2.wrapping_add(K3_OFF)) ^ k > XOR_LIMIT
                    && rd8(lf_checker_rt::relocated(GLOBAL_A)) == 0
                {
                    refresh = true;
                }
            }
        }
        if refresh {
            if rd8(lf_checker_rt::relocated(GLOBAL_B)) == 0 {
                let base = rd32(this.wrapping_add(WEIGHT_OFF));
                wr32(base.wrapping_add(WEIGHT_SLOT), ONE_BITS);
                if (a1 as u8) != 0 {
                    let c = rd8(this.wrapping_add(CNT_A)).wrapping_add(1);
                    wr8(this.wrapping_add(CNT_A), c);
                    if c > 1 {
                        wr8(this.wrapping_add(CNT_A), 0);
                    }
                } else if (a5 as u8) != 0 {
                    let c = rd8(this.wrapping_add(CNT_B)).wrapping_add(1);
                    wr8(this.wrapping_add(CNT_B), c);
                    if c > 2 {
                        wr8(this.wrapping_add(CNT_B), 0);
                    }
                } else if !busy {
                    let c = rd8(this.wrapping_add(CNT_C)).wrapping_add(1);
                    wr8(this.wrapping_add(CNT_C), c);
                    if c > 2 {
                        wr8(this.wrapping_add(CNT_C), 0);
                    }
                }
            }
        } else if !ok {
            status_clear(this);
        } else if (a4 as u8) == 0
            && rd8(lf_checker_rt::relocated(GLOBAL_C)) != 0
            && a2 != 0
        {
            let sub2 = rd32(this.wrapping_add(SUB_OFF));
            let idx = rd32(sub2.wrapping_add(COUNT_OFF));
            let slot = lf_checker_rt::relocated(FLOAT_TABLE)
                .wrapping_add(idx.wrapping_mul(4));
            let x = f32::from_bits(rd32(slot));
            let xref = f32::from_bits(rd32(lf_checker_rt::relocated(FLOAT_REF)));
            // The original continues unless the comparison is ordered-equal
            // (lahf/test/jnp over ucomiss flags); `!=` matches exactly,
            // including the unordered (NaN) case which also continues.
            if x != xref {
                let dl = rd8(a2.wrapping_add(DL_OFF));
                let mut ask = dl == 0;
                if !ask {
                    let first =
                        rd8(a2.wrapping_add(P1B_OFF)) ^ rd8(a2.wrapping_add(P1A_OFF));
                    if first > XOR_LIMIT {
                        status_set(this);
                    } else {
                        let second = rd8(a2.wrapping_add(P2B_OFF))
                            ^ rd8(a2.wrapping_add(P2A_OFF));
                        if second > XOR_LIMIT {
                            status_clear(this);
                        } else {
                            ask = true;
                        }
                    }
                }
                if ask {
                    let r4: u32 = lf_checker_rt::callee_thiscall!(
                        4,
                        u32,
                        a2.wrapping_add(ID4_BIAS)
                    );
                    if (r4 as u8) != 0 {
                        let s = rd8(this.wrapping_add(STATUS_OFF));
                        wr8(this.wrapping_add(STATUS_OFF), s ^ 1);
                    }
                }
            }
        }
        if !ok {
            status_clear(this);
        }
        0
    }
});
