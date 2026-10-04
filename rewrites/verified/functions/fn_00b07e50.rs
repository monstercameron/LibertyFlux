// original: 0x00B07E50 net_session_state_update (proposed)
//
// Advance one network session tick: scan the session's node list, pick the
// next sub-state, refresh the peer snapshots, and select the follow-up
// action. Always returns 1.
//
// `this` points to the session object; its current sub-state is the dword at
// `+0x140`. Callee 1 (thiscall on `this`, no stack arguments) returns the
// session context. Bit 2 of the dword at context `+0x26c` seeds the search,
// and when it is set the pointer at context `+0xb30` (with a marker dword
// `3` at its `+0x1304`) arms a flag.
//
// The node list hangs off context `+0x224` (`+0x2e0` past that); nodes link
// through `+0xc` and carry a tag at `+0x4` and a 3-bit state at `+0x8`
// (bits 1-3). The first scan looks for tag `0x2de`: the original recomputes
// the same state value into two registers and compares them, so its
// early-abort arm is dead and the scan always runs to the end. A hit calls
// callee 2 (thiscall, `0x2de` and `5`); a small answer (minus `0x18` at most
// `5`) forces sub-state `2`, a large one keeps `3`. On a miss the second
// scan looks for tag `0x2e2` with a live abort (a rising state from `2` or
// more ends the walk); a hit with the flag clear forces sub-state `0`, with
// the flag set (or no hit at all) the sub-state is `3` for a clear seed bit
// and `1` for a set one. A sub-state above `3` skips the switch.
//
// The sub-state picks one of four small tables indexed by the session mode
// (`[this+0x140]`, `0-3`; anything higher keeps selector `1`):
// sub-state 0 yields `0/8/6/1`, 1 yields `2/0/0/2`, 2 yields `6/1/0/7`,
// 3 yields `1/3/3/0` for modes `0/1/2/3`.
//
// Two peer snapshots are then fetched (callee 3 with `(1, 0)`, callee 4
// with `(2, 0)`, both thiscalls on `this`). In mode `2` with both present,
// four words are copied from the first snapshot (`+0x20-0x2c`) to the second
// (`+0x150-0x15c`, the middle two as bit copies) and its byte at `+0x141`
// is set. In mode `0` with both present and bit `0x4` of the first
// snapshot's byte at `+0x38e` clear (and a saved sub-state other than `3`),
// callee 5 (thiscall on the first snapshot, the two words at second
// `+0x190/+0x194`) runs and bit `0x8` is set; the global activity flag is
// then set. Otherwise, when the first snapshot exists and the activity flag
// is set, the flag is cleared and callee 6 (cdecl `(0, 0)`) supplies a
// reader: two absolute readings through it (thiscall/cdecl pairs 7/8 and
// 9/10) set a near flag when either exceeds `10`. A virtual call (slot
// `+0xec`) on the context's `+0xb30` object, when present, returns a float
// triple whose length past `0.2` sets a far flag and skips the second one;
// otherwise the same slot on the context itself is measured the same way.
// With neither flag, a context marker
// other than `6` at `+0x7b8`, a zero low byte from callee 13 (thiscall on
// `this+0x40`) and a nonzero dword at `this+0x130`, callee 14 (thiscall on
// `this` with `(0, 0x3e8, 1, 1)`) runs.
//
// The tail maps the selector through a countdown: selector `8` reloads the
// global counter from its source and continues with `0`; a positive counter
// is decremented (reaching `0` continues with `8` instead). The selector
// minus `2` indexes the action table: `0/1/3/5/6` call the follow-up
// callees 15/16/17/18/19 (one per slot; slot `3` needs selector `5`, which
// no table yields, so callee 17 never fires), anything else runs no call.
// Every path stores the saved sub-state back to `+0x140` and returns `1`.
//
// Original: 0x00B07E50 (thiscall, no stack arguments, `eax` return, always 1).
lf_checker_rt::export!(thiscall, rw_00B07E50(this: u32) -> u32 {
    unsafe {
        const CAL_CTX: u32 = 1;
        const CAL_FIND: u32 = 2;
        const CAL_SNAP_A: u32 = 3;
        const CAL_SNAP_B: u32 = 4;
        const CAL_SYNC: u32 = 5;
        const CAL_READER: u32 = 6;
        const CAL_M_A: u32 = 7;
        const CAL_C_A: u32 = 8;
        const CAL_M_B: u32 = 9;
        const CAL_C_B: u32 = 10;
        const CAL_VEC_A: u32 = 11;
        const CAL_VEC_B: u32 = 12;
        const CAL_PROBE: u32 = 13;
        const CAL_NOTIFY: u32 = 14;
        const CAL_ACT_0: u32 = 15;
        const CAL_ACT_1: u32 = 16;
        const CAL_ACT_3: u32 = 17;
        const CAL_ACT_5: u32 = 18;
        const CAL_ACT_6: u32 = 19;
        const VT_SLOT: u32 = 0xec;
        const CTX_SEED: u32 = 0x26c;
        const CTX_LIST: u32 = 0x224;
        const CTX_AUX: u32 = 0xb30;
        const CTX_MARK: u32 = 0x7b8;
        const LIST_HEAD: u32 = 0x2e0;
        const NODE_TAG: u32 = 0x4;
        const NODE_STATE: u32 = 0x8;
        const NODE_NEXT: u32 = 0xc;
        const TAG_FIRST: u32 = 0x2de;
        const TAG_SECOND: u32 = 0x2e2;
        const AUX_MARK: u32 = 0x1304;
        const AUX_ARMED: u32 = 3;
        const SES_MODE: u32 = 0x140;
        const SES_COUNT: u32 = 0x130;
        const SNAP_WORDS: u32 = 0x20;
        const DST_WORDS: u32 = 0x150;
        const DST_FLAG: u32 = 0x141;
        const SYNC_ARGS: u32 = 0x190;
        const SNAP_CTL: u32 = 0x38e;
        const G_ACTIVE: u32 = 0x16154A2;
        const G_COUNTER: u32 = 0x16154D4;
        const G_SOURCE: u32 = 0x1040094;
        const K_LEN: f32 = f32::from_bits(0x3e4ccccd); // 0.2
        // Sub-state tables by session mode (rows: sub-state 0..3).
        const SEL: [[u32; 4]; 4] = [
            [0, 8, 6, 1],
            [2, 0, 0, 2],
            [6, 1, 0, 7],
            [1, 3, 3, 0],
        ];

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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
        /// Absolute value as the original's cdq/xor/sub sequence forms it
        /// (`i32::MIN` stays `i32::MIN`), compared signed against 10: the
        /// original uses `jg`/`jle`, so `i32::MIN` does not count as big.
        #[inline(always)]
        fn big_abs(v: u32) -> bool {
            (v as i32).wrapping_abs() > 10
        }
        /// Call the virtual slot on `obj`, handing it a scratch pointer,
        /// and report whether the returned triple's length exceeds the limit.
        unsafe fn far(obj: u32, _id: u32) -> bool {
            unsafe {
                let vt = rd32(obj);
                let target = rd32(vt + VT_SLOT);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                let mut buf = [0u32; 3];
                let p = f(obj, buf.as_mut_ptr() as u32);
                let x = rdf(p);
                let y = rdf(p.wrapping_add(4));
                let z = rdf(p.wrapping_add(8));
                let len = add(add(mul(x, x), mul(y, y)), mul(z, z)).sqrt();
                len > K_LEN
            }
        }

        let ctx = lf_checker_rt::callee_thiscall!(CAL_CTX, u32, this);
        let bit = (rd32(ctx.wrapping_add(CTX_SEED)) >> 2) & 1;
        let aux = if bit == 0 {
            0
        } else {
            rd32(ctx.wrapping_add(CTX_AUX))
        };
        let armed = bit != 0 && aux != 0 && rd32(aux.wrapping_add(AUX_MARK)) == AUX_ARMED;
        // First scan for TAG_FIRST (the early-abort arm is dead: both
        // compared values come from the same node word).
        let head = rd32(rd32(ctx.wrapping_add(CTX_LIST)).wrapping_add(LIST_HEAD));
        let sub: u32;
        let mut node = head;
        let mut found_first = false;
        if node != 0 {
            loop {
                if rd32(node.wrapping_add(NODE_TAG)) == TAG_FIRST {
                    found_first = true;
                    break;
                }
                node = rd32(node.wrapping_add(NODE_NEXT));
                if node == 0 {
                    break;
                }
            }
        }
        if found_first {
            let base = rd32(ctx.wrapping_add(CTX_LIST)).wrapping_add(LIST_HEAD);
            let r = lf_checker_rt::callee_thiscall!(CAL_FIND, u32, base, TAG_FIRST, 5)
                .wrapping_sub(0x18);
            sub = if r > 5 { 3 } else { 2 };
        } else if head != 0 {
            // Second scan for TAG_SECOND with the live rising-state abort.
            let mut prev = (rd32(head.wrapping_add(NODE_STATE)) >> 1) & 7;
            let mut cur = head;
            let mut found_second = false;
            loop {
                let st = (rd32(cur.wrapping_add(NODE_STATE)) >> 1) & 7;
                if prev < st && prev >= 2 {
                    break;
                }
                prev = st;
                if rd32(cur.wrapping_add(NODE_TAG)) == TAG_SECOND {
                    found_second = true;
                    break;
                }
                cur = rd32(cur.wrapping_add(NODE_NEXT));
                if cur == 0 {
                    break;
                }
            }
            if found_second && !armed {
                sub = 0;
            } else {
                sub = if bit == 0 { 3 } else { 1 };
            }
        } else {
            sub = if bit == 0 { 3 } else { 1 };
        }
        let saved = sub;
        // Selector from the sub-state/mode tables.
        let mut sel: u32 = 1;
        if sub <= 3 {
            let mode = rd32(this.wrapping_add(SES_MODE));
            if mode <= 3 {
                sel = SEL[sub as usize][mode as usize];
            }
        }
        let snap_a = lf_checker_rt::callee_thiscall!(CAL_SNAP_A, u32, this, 1, 0);
        let snap_b = lf_checker_rt::callee_thiscall!(CAL_SNAP_B, u32, this, 2, 0);
        let active = lf_checker_rt::global::<u8>(G_ACTIVE);
        let counter = lf_checker_rt::global::<u32>(G_COUNTER);
        let source = lf_checker_rt::global::<u32>(G_SOURCE);
        let mode = rd32(this.wrapping_add(SES_MODE));
        if mode == 2 && snap_a != 0 && snap_b != 0 {
            wr8(snap_b.wrapping_add(DST_FLAG), 1);
            wr32(snap_b.wrapping_add(DST_WORDS), rd32(snap_a.wrapping_add(SNAP_WORDS)));
            wr32(
                snap_b.wrapping_add(DST_WORDS + 4),
                rd32(snap_a.wrapping_add(SNAP_WORDS + 4)),
            );
            wr32(
                snap_b.wrapping_add(DST_WORDS + 8),
                rd32(snap_a.wrapping_add(SNAP_WORDS + 8)),
            );
            wr32(
                snap_b.wrapping_add(DST_WORDS + 12),
                rd32(snap_a.wrapping_add(SNAP_WORDS + 12)),
            );
        }
        let mode = rd32(this.wrapping_add(SES_MODE));
        if mode == 0 {
            if snap_a == 0 {
                active.write(0);
            } else if snap_b != 0 {
                if rd8(snap_a.wrapping_add(SNAP_CTL)) & 4 == 0 {
                    if saved != 3 {
                        let f0 = rd32(snap_b.wrapping_add(SYNC_ARGS));
                        let f1 = rd32(snap_b.wrapping_add(SYNC_ARGS + 4));
                        lf_checker_rt::callee_thiscall!(CAL_SYNC, u32, snap_a, f0, f1);
                    }
                    wr8(
                        snap_a.wrapping_add(SNAP_CTL),
                        rd8(snap_a.wrapping_add(SNAP_CTL)) | 8,
                    );
                }
                active.write(1);
            } else if snap_a != 0 {
                // snap_b == 0 with snap_a present: same handling as a
                // nonzero mode below.
                if active.read() != 0 {
                    active.write(0);
                    let r = lf_checker_rt::callee_cdecl!(CAL_READER, u32, 0, 0);
                    let mut near = false;
                    if r != 0 {
                        let ma = lf_checker_rt::callee_thiscall!(CAL_M_A, u32, r);
                        let ca = lf_checker_rt::callee_cdecl!(CAL_C_A, u32, ma);
                        if big_abs(ca) {
                            near = true;
                        } else {
                            let mb = lf_checker_rt::callee_thiscall!(CAL_M_B, u32, r);
                            let cb = lf_checker_rt::callee_cdecl!(CAL_C_B, u32, mb);
                            near = big_abs(cb);
                        }
                    }
                    // A far first triple skips the second call.
                    let mut far_flag = false;
                    if aux == 0 || !far(aux, CAL_VEC_A) {
                        if far(ctx, CAL_VEC_B) {
                            far_flag = true;
                        }
                    } else {
                        far_flag = true;
                    }
                    if !near
                        && !far_flag
                        && rd32(ctx.wrapping_add(CTX_MARK)) != 6
                        && lf_checker_rt::callee_thiscall!(
                            CAL_PROBE,
                            u32,
                            this.wrapping_add(0x40)
                        ) & 0xff
                            == 0
                        && rd32(this.wrapping_add(SES_COUNT)) != 0
                    {
                        lf_checker_rt::callee_thiscall!(CAL_NOTIFY, u32, this, 0, 0x3e8, 1, 1);
                    }
                }
            } else {
                active.write(0);
            }
        } else if snap_a == 0 {
            active.write(0);
        } else if active.read() != 0 {
            active.write(0);
            let r = lf_checker_rt::callee_cdecl!(CAL_READER, u32, 0, 0);
            let mut near = false;
            if r != 0 {
                let ma = lf_checker_rt::callee_thiscall!(CAL_M_A, u32, r);
                let ca = lf_checker_rt::callee_cdecl!(CAL_C_A, u32, ma);
                if big_abs(ca) {
                    near = true;
                } else {
                    let mb = lf_checker_rt::callee_thiscall!(CAL_M_B, u32, r);
                    let cb = lf_checker_rt::callee_cdecl!(CAL_C_B, u32, mb);
                    near = big_abs(cb);
                }
            }
            // A far first triple skips the second call.
            let mut far_flag = false;
            if aux == 0 || !far(aux, CAL_VEC_A) {
                if far(ctx, CAL_VEC_B) {
                    far_flag = true;
                }
            } else {
                far_flag = true;
            }
            if !near
                && !far_flag
                && rd32(ctx.wrapping_add(CTX_MARK)) != 6
                && lf_checker_rt::callee_thiscall!(CAL_PROBE, u32, this.wrapping_add(0x40)) & 0xff
                    == 0
                && rd32(this.wrapping_add(SES_COUNT)) != 0
            {
                lf_checker_rt::callee_thiscall!(CAL_NOTIFY, u32, this, 0, 0x3e8, 1, 1);
            }
        }
        // Tail: countdown through the action table.
        if sel == 8 {
            let v = source.read();
            counter.write(v);
            sel = 0;
        }
        let mut ctr = counter.read();
        if (ctr as i32) > 0 {
            ctr = ctr.wrapping_sub(1);
            counter.write(ctr);
            if ctr == 0 {
                sel = 8;
            }
        }
        match sel.wrapping_sub(2) {
            0 => {
                lf_checker_rt::callee_thiscall!(CAL_ACT_0, u32, this);
            }
            1 => {
                lf_checker_rt::callee_thiscall!(CAL_ACT_1, u32, this);
            }
            3 => {
                lf_checker_rt::callee_thiscall!(CAL_ACT_3, u32, this);
            }
            5 => {
                lf_checker_rt::callee_thiscall!(CAL_ACT_5, u32, this);
            }
            6 => {
                lf_checker_rt::callee_thiscall!(CAL_ACT_6, u32, this);
            }
            _ => {}
        }
        wr32(this.wrapping_add(SES_MODE), saved);
        1
    }
});
