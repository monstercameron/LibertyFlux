// original: 0x00b76c90 ped_task_state_dispatch (proposed)

/// Dispatch a ped task state machine step on its state byte.
///
/// `this` points at the task, `a0` is a tick value kept in the stamp slot
/// on every exit, and word 1's low byte is a quiet flag. The state byte at
/// `+0xbc8` minus one selects four cases (anything else just re-stamps):
///
/// - State 1 settles the accumulator against its limit (possibly refreshing
///   through a callee) and releases all nine live link slots.
/// - State 2 runs the request starter (function `0x00b769a0`) over the
///   task's own fields: answer 1 continues into a target chain (resolve,
///   two queries, a four-word register call, then a subtask release unless
///   quiet, which also advances the state to 3); answer 2 refreshes and
///   exits; anything else exits.
/// - State 3 rebuilds the candidate set from the subtask (a descriptor
///   callee plus one creator call per empty link slot), checks two idle
///   flags, resolves a key through a lookup chain (8, then 0xd), collects
///   up to two ids through a second chain (0xc) into scratch, looks the
///   winner up in a registry (with one retry), then walks the registry
///   entry's rows: a row matching the first id converts its float with
///   x87 truncation semantics and appends the pair to the candidate list,
///   a row matching the second id stores its value into one of four slots
///   by a second match. The candidate list is bubble-sorted by low word.
/// - State 4 folds the subtask's value into the accumulator (signed
///   saturation-like update), releases the subtask, notifies a busy
///   target, and resets the state to 1.
///
/// The x87 conversion loads an f32, truncates toward zero to i64, and keeps
/// the low 32 bits (`0x80000000` out of range, NaN included). No defined
/// return value.
///
/// The state-2 target chain is present below but excluded from the proof:
/// the original pushes four words for its register call yet cleans twenty
/// bytes (`(an instruction of the original)`), so taking that branch returns into the stamp
/// word and crashes; the checker contract never takes it. See the lane
/// report for the evidence.
///
/// Original: 0x00b76c90 (thiscall, two stack words).
unsafe fn run_00b76c90(this: u32, a0: u32, a1: u32, dispatch_sub: u32) -> u32 {
    unsafe {
        const REGISTRY: u32 = 0x0115dc18;
        const CREATOR_REG: u32 = 0x012845d0;
        const EXTRA_ARG: u32 = 0x00eb243c;
        const LOOKUP_MAGIC: u32 = 0x7deef8b7;
        const GATE_CREATE: u32 = 0x01046ee4;
        const ID_A: u32 = 1;
        const ID_B: u32 = 2;
        const ID_START: u32 = 3;
        const ID_D: u32 = 4;
        const ID_E: u32 = 5;
        const ID_F: u32 = 6;
        const ID_G: u32 = 7;
        const ID_H: u32 = 8;
        const ID_I: u32 = 9;
        const ID_J: u32 = 10;
        const ID_K: u32 = 11;
        const ID_L: u32 = 12;
        const ID_M: u32 = 13;
        const ID_N: u32 = 14;
        const ID_O: u32 = 15;
        const ID_P: u32 = 16;
        const ID_Q: u32 = 17;
        const ID_R: u32 = 18;
        const ID_S: u32 = 19;
        const ID_U: u32 = 20;
        const GA8: u32 = 0x0167cca8;
        const GB0: u32 = 0x0167ccb0;
        const GB8: u32 = 0x0167ccb8;
        const GAC: u32 = 0x0167ccac;
        const GBC: u32 = 0x0167ccbc;
        const GA4: u32 = 0x0167cca4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        /// Low 32 bits of an f32 converted with x87 `fistp qword` under a
        /// truncating control word: truncate toward zero, `0x80000000`
        /// when the result does not fit in an i64 (NaN included).
        #[inline(always)]
        fn fistp_low(x: f32) -> u32 {
            let t = x.trunc();
            if t >= -9223372036854775808.0 && t < 9223372036854775808.0 {
                (t as i64) as u32
            } else {
                0x80000000
            }
        }

        let quiet = ((a1 & 0xff) as u8) != 0;
        let case = (rd8(this.wrapping_add(0xbc8)) as u32).wrapping_sub(dispatch_sub);
        if case > 3 {
            wr32(this.wrapping_add(0x998), a0);
            return 0;
        }
        match case {
            0 => {
                if !quiet {
                    if rd32(this.wrapping_add(0x998)) == 0 {
                        wr32(this.wrapping_add(0x998), a0);
                    }
                    let d = a0.wrapping_sub(rd32(this.wrapping_add(0x998)));
                    wr32(this.wrapping_add(0x990), rd32(this.wrapping_add(0x990)).wrapping_add(d));
                    if !((rd32(this.wrapping_add(0x990)) as i32) < (rd32(this.wrapping_add(0x994)) as i32)) {
                        lf_checker_rt::callee_thiscall!(ID_A, u32, this);
                    }
                }
                let mut k = 0u32;
                while k < 9 {
                    let slot = rd32(this.wrapping_add(0x968).wrapping_add(k.wrapping_mul(4)));
                    if slot != 0 {
                        lf_checker_rt::callee_thiscall!(ID_B, u32, slot, 0);
                    }
                    k += 1;
                }
                wr32(this.wrapping_add(0x998), a0);
            }
            1 => {
                let r = lf_checker_rt::callee_thiscall!(
                    ID_START, u32, this,
                    rd32(this.wrapping_add(0x98c)),
                    rd32(this.wrapping_add(0x990)),
                    rd8(this.wrapping_add(0xbc9)) as u32,
                    1
                );
                if r == 1 {
                    // Excluded from the proof (see doc comment): the
                    // original crashes on this branch. Mirrored here for
                    // faithfulness; never executed under the contract.
                    if rd8(this.wrapping_add(0xbcc)) != 0 {
                        let tgt = rd32(this.wrapping_add(0x98c));
                        let h = lf_checker_rt::callee_thiscall!(ID_D, u32, tgt);
                        wr32(this.wrapping_add(0x994), h);
                        let e = lf_checker_rt::callee_thiscall!(ID_E, u32, tgt);
                        let f = lf_checker_rt::callee_thiscall!(ID_F, u32, tgt, e);
                        lf_checker_rt::callee_cdecl!(ID_G, u32, this.wrapping_add(0xab8), 0xff, lf_checker_rt::relocated(EXTRA_ARG), f);
                    }
                    if quiet {
                        wr32(this.wrapping_add(0x998), a0);
                        return 0;
                    }
                    lf_checker_rt::callee_thiscall!(ID_H, u32, rd32(this.wrapping_add(0x964)));
                    wr32(this.wrapping_add(0x998), a0);
                    wr8(this.wrapping_add(0xbc8), 3);
                } else if r == 2 {
                    lf_checker_rt::callee_thiscall!(ID_A, u32, this);
                    wr32(this.wrapping_add(0x998), a0);
                } else {
                    wr32(this.wrapping_add(0x998), a0);
                }
            }
            2 => {
                let sub = rd32(this.wrapping_add(0x964));
                if sub == 0 {
                    lf_checker_rt::callee_thiscall!(ID_A, u32, this);
                    wr32(this.wrapping_add(0x998), a0);
                    return 0;
                }
                wr32(this.wrapping_add(0x990), rd32(sub.wrapping_add(0xb8)));
                let mut desc = [0u32; 32];
                let dp = desc.as_mut_ptr() as u32;
                lf_checker_rt::callee_cdecl!(ID_I, u32,);
                let mut k = 0u32;
                while k < 9 {
                    let slotaddr = this.wrapping_add(0x968).wrapping_add(k.wrapping_mul(4));
                    if rd32(slotaddr) == 0 {
                        lf_checker_rt::callee_thiscall!(
                            ID_J, u32, lf_checker_rt::relocated(CREATOR_REG),
                            rd32(lf_checker_rt::relocated(GATE_CREATE)),
                            slotaddr, dp, 0xffffffff, 0, 0
                        );
                    }
                    k += 1;
                }
                if rd8(this.wrapping_add(0xbcc)) != 0 || rd8(this.wrapping_add(0xbcb)) != 0 {
                    wr32(this.wrapping_add(0x998), a0);
                    return 0;
                }
                let key = rd32(rd32(this.wrapping_add(0x960)).wrapping_add(4));
                let mut desc2 = [0u32; 32];
                let dp2 = desc2.as_mut_ptr() as u32;
                lf_checker_rt::callee_cdecl!(ID_K, u32,);
                let mut w14 = 0xffffffffu32;
                let mut w18 = 0xffffffffu32;
                let mut bx = 0u32;
                let mut have_bx = false;
                let t = lf_checker_rt::callee_thiscall!(ID_L, u32, lf_checker_rt::relocated(REGISTRY), key);
                if t != 0 && rd8(t) == 8 {
                    let p = lf_checker_rt::callee_cdecl!(ID_M, u32, t, dp2);
                    let n = lf_checker_rt::callee_thiscall!(ID_N, u32, lf_checker_rt::relocated(REGISTRY), rd32(p.wrapping_add(5)));
                    if n != 0 && rd8(n) == 0x0d {
                        let o = lf_checker_rt::callee_cdecl!(ID_O, u32, n, dp2);
                        let mut k = 0u32;
                        while k < 2 {
                            let q = lf_checker_rt::callee_thiscall!(ID_P, u32, lf_checker_rt::relocated(REGISTRY), rd32(o.wrapping_add(1).wrapping_add(k.wrapping_mul(8))));
                            if q != 0 && rd8(q) == 0x0c {
                                let s = lf_checker_rt::callee_cdecl!(ID_Q, u32, q, dp2);
                                let v = rd32(s.wrapping_add(8));
                                if k == 0 {
                                    w14 = v;
                                } else {
                                    w18 = v;
                                }
                            }
                            k += 1;
                        }
                        if w14 != 0xffffffff || w18 != 0xffffffff {
                            bx = lf_checker_rt::callee_cdecl!(ID_R, u32, rd32(this.wrapping_add(0x98c)), w14, LOOKUP_MAGIC);
                            if bx == 0 {
                                bx = lf_checker_rt::callee_cdecl!(ID_S, u32, rd32(this.wrapping_add(0x98c)), w18, LOOKUP_MAGIC);
                                have_bx = bx != 0;
                            } else {
                                have_bx = true;
                            }
                        }
                    }
                }
                if have_bx && rd32(bx.wrapping_add(4)) & 0xfffffff0 != 0 {
                    let count = rd32(bx.wrapping_add(4)) >> 4;
                    let mut dx = bx.wrapping_add(0x14);
                    let mut si = 0u32;
                    while si < count {
                        let id = rd32(dx.wrapping_sub(0x0c));
                        if id == rd32(lf_checker_rt::relocated(GA8)) {
                            let c = rd32(this.wrapping_add(0xaa0));
                            if c.wrapping_add(1) < 0x20 {
                                let lo = fistp_low(rdf(dx.wrapping_sub(8)));
                                wr32(this.wrapping_add(c.wrapping_mul(8)).wrapping_add(0x9a4), lo);
                                let c2 = rd32(this.wrapping_add(0xaa0));
                                wr32(this.wrapping_add(c2.wrapping_mul(8)).wrapping_add(0x9a0), rd32(dx));
                                wr32(this.wrapping_add(0xaa0), c2.wrapping_add(1));
                            }
                        } else if id == rd32(lf_checker_rt::relocated(GB0)) {
                            let f = rd32(dx.wrapping_sub(8));
                            if f == rd32(lf_checker_rt::relocated(GB8)) {
                                wr32(this.wrapping_add(0xaa4), rd32(dx));
                            } else if f == rd32(lf_checker_rt::relocated(GAC)) {
                                wr32(this.wrapping_add(0xaa8), rd32(dx));
                            } else if f == rd32(lf_checker_rt::relocated(GBC)) {
                                wr32(this.wrapping_add(0xaac), rd32(dx));
                            } else if f == rd32(lf_checker_rt::relocated(GA4)) {
                                wr32(this.wrapping_add(0xab0), rd32(dx));
                            }
                        }
                        si += 1;
                        dx = dx.wrapping_add(0x10);
                    }
                }
                if rd32(this.wrapping_add(0xaa0)) != 0 {
                    loop {
                        let m = rd32(this.wrapping_add(0xaa0));
                        if m.wrapping_sub(1) == 0 {
                            break;
                        }
                        let mut swapped = false;
                        let mut dx = this.wrapping_add(0x9a0);
                        let mut si = 0u32;
                        while si < m.wrapping_sub(1) {
                            let b = rd32(dx);
                            let e = rd32(dx.wrapping_add(8));
                            if b > e {
                                let c = rd32(dx.wrapping_add(4));
                                let a = rd32(dx.wrapping_add(12));
                                wr32(dx, e);
                                wr32(dx.wrapping_add(12), c);
                                wr32(dx.wrapping_add(4), a);
                                wr32(dx.wrapping_add(8), b);
                                swapped = true;
                            }
                            si += 1;
                            dx = dx.wrapping_add(8);
                        }
                        if !swapped {
                            break;
                        }
                    }
                }
                wr32(this.wrapping_add(0x998), a0);
                wr8(this.wrapping_add(0xbcb), 1);
            }
            _ => {
                let sub = rd32(this.wrapping_add(0x964));
                if sub != 0 {
                    let si = rd32(this.wrapping_add(0x99c));
                    let dx = rd32(sub.wrapping_add(0xb8));
                    wr32(this.wrapping_add(0x990), dx);
                    if !((si.wrapping_neg() as i32) >= (dx as i32)) {
                        wr32(this.wrapping_add(0x990), si.wrapping_add(dx));
                        wr32(this.wrapping_add(0x99c), 0);
                    }
                    lf_checker_rt::callee_thiscall!(ID_B, u32, sub, 0);
                }
                if rd8(this.wrapping_add(0xbcc)) != 0 {
                    let c = rd32(this.wrapping_add(0x98c));
                    if c != 0 && rd32(c.wrapping_add(0x98)) != 0 {
                        lf_checker_rt::callee_thiscall!(ID_U, u32, c);
                    }
                }
                wr8(this.wrapping_add(0xbc8), 1);
                wr32(this.wrapping_add(0x998), a0);
            }
        }
        0
    }
}

lf_checker_rt::export!(thiscall, rw_00b76c90(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe { run_00b76c90(this, a0, a1, 1) }
});
