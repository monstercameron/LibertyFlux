// original: 0x009F33D0 CPlayerPed::vf31

/// Refresh a player ped from a target point: range-gate, relatch, sweep the
/// task head, and finish through the notifier.
///
/// `this` is the player ped (thiscall); `point` addresses three input
/// floats copied to locals; the second stack argument is never read; only
/// the low byte of `flags` is tested.
///
/// The target is aligned when `+TARGET` is non-null and equals `+LATCHED`.
/// Unless skipped by `flags`, the range block runs when `+RANGE_ON` is set:
/// the virtual probe (slot `+VT_PROBE` of the object at `+QUEUE` plus
/// `PROBE_OFF`) runs, and unless it answers `PROBE_SKIP`, callee A latches
/// the queue when the guard object (`+GUARD`) is missing or its flag byte
/// (`+GUARD_FLAG`) is clear. The distance block then runs whenever
/// `+RANGE_ON` is set: it accumulates the squared distance between the
/// copied point and the anchor row (`+MATRIX` plus `ROW_X/Y/Z`) in the
/// original's exact operation order and, when strictly greater than the
/// global threshold (ordered comparison, so a NaN on either side skips),
/// runs callee B and hands the copied point to callee C by pointer (the
/// pointed-to words are snap-compared; the address itself is skipped).
/// Either way the marker `MARK_RANGE` is set.
///
/// The latch block runs callee D when aligned, and when aligned also clears
/// `AUX_BIT` in the auxiliary object (`+AUX`) and runs callee E. A set
/// slot (`+SLOT`) is then drained through callee F and cleared, the status
/// word (`+STATUS`) has `STATUS_BIT` cleared, and nine scratch words are
/// zeroed. Three virtual slots fire unconditionally: `+VT_OUT` with the
/// copied point by pointer (snap-compared like callee C), `+VT_FLOAT` with
/// the bits of `+FLOAT`, and `+VT_ONE` with 1; then `+SEEN` is set.
///
/// The dispatch block reads the target's index word: unless null or
/// `NO_TARGET`, it addresses a row of the global dispatch table and runs
/// callee H when that row's tag equals `TAG_WANT`.
///
/// The sweep block runs callee I (skipping the rest on a null answer),
/// then twice more with callee J and callee K applied to the matrix word
/// (`+MATRIX`), then the answer's own slot `+VT_STEP`, and when that
/// answer's selector word (`+SELECT`) is non-zero, callee L with the
/// selector and a zero tag.
///
/// The head block runs when the head (`+LATCHED`) is live and mismatched
/// and the head state (`+HEAD_STATE`) is neither `STATE_A` nor `STATE_B`:
/// the head's slot `+VT_HEAD` runs with the head plus `HEAD_OFF`, callee M
/// runs with the head and the matrix word, the head slot runs again,
/// callee N runs, the head's slot `+VT_STEP2` runs twice, and the probe
/// float (`+PROBE_LINK` chain, `+PROBE_VAL`) is saved, callee O runs, and
/// the saved float is compared against the live one with ordered-equal
/// semantics (a NaN on either side, which is also how the harness reaches
/// the unequal path, counts as unequal): on equality callee Q runs with the
/// head index, otherwise callee P runs with the head index and the saved
/// float bits.
///
/// Finally flag `TAIL_BIT` in `+MODE` is cleared and callee R runs with the
/// relocated service object and the ped.
///
/// Returns nothing; the original is void.
lf_checker_rt::export!(thiscall, rw_009F33D0(this: u32, point: u32, _unused: u32, flags: u32) -> u32 {
    unsafe {
        const TARGET: u32 = 0x38;
        const LATCHED: u32 = 0x7b4;
        const NO_TARGET: u16 = 0xffff;
        const RANGE_ON: u32 = 0x219;
        const QUEUE: u32 = 0x224;
        const PROBE_OFF: u32 = 0x54;
        const VT_PROBE: u32 = 0x0c;
        const PROBE_SKIP: u32 = 3;
        const GUARD: u32 = 0x6c;
        const GUARD_FLAG: u32 = 0x0e;
        const MATRIX: u32 = 0x20;
        const ROW_X: u32 = 0x30;
        const ROW_Y: u32 = 0x34;
        const ROW_Z: u32 = 0x38;
        const THRESH: u32 = 0x00fe_8b40;
        const MARK_RANGE: u32 = 0xbe0;
        const MARK_RANGE_BITS: u32 = 0x0c;
        const AUX: u32 = 0x78;
        const AUX_BIT: u32 = 2;
        const SLOT: u32 = 0x16c;
        const STATUS: u32 = 0x26c;
        const STATUS_BIT: u32 = 1;
        const VT_OUT: u32 = 0x08;
        const FLOAT: u32 = 0xaa0;
        const VT_FLOAT: u32 = 0x0c;
        const VT_ONE: u32 = 0xb4;
        const SEEN: u32 = 0xbe;
        const DISPATCH: u32 = 0x012b_9c78;
        const DISPATCH_ROWS: u32 = 0x70;
        const VT_STEP: u32 = 0x0c;
        const SELECT: u32 = 0x18;
        const HEAD_STATE: u32 = 0x7b8;
        const STATE_A: u32 = 6;
        const STATE_B: u32 = 3;
        const VT_HEAD: u32 = 0x10;
        const HEAD_OFF: u32 = 0x10;
        const VT_STEP2: u32 = 0xbc;
        const PROBE_LINK: u32 = 0x04;
        const PROBE_NEXT: u32 = 0x0c;
        const PROBE_VAL: u32 = 0x08;
        const MODE: u32 = 0x118;
        const TAIL_BIT: u32 = 0x80000;
        const SERVICE: u32 = 0x0139_4d60;
        const CAL_LATCH: u32 = 1;
        const CAL_FAR: u32 = 2;
        const CAL_HANDOFF: u32 = 3;
        const CAL_RELATCH: u32 = 4;
        const CAL_ALIGN: u32 = 5;
        const CAL_DRAIN: u32 = 6;
        const CAL_SYNC: u32 = 7;
        const CAL_DISPATCH: u32 = 8;
        const CAL_SWEEP: u32 = 9;
        const CAL_APPLY_A: u32 = 10;
        const CAL_APPLY_B: u32 = 11;
        const CAL_SELECT: u32 = 12;
        const CAL_SAMPLE: u32 = 13;
        const CAL_HEAD_SYNC: u32 = 14;
        const CAL_PROBE_RUN: u32 = 15;
        const CAL_UNEQUAL: u32 = 16;
        const CAL_EQUAL: u32 = 17;
        const CAL_NOTIFY: u32 = 18;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn rd_global(file_va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() }
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

        let f0 = rdf(point);
        let f1 = rdf(point.wrapping_add(4));
        let f2 = rdf(point.wrapping_add(8));
        let tgt = rd32(this.wrapping_add(TARGET));
        let aligned = tgt != 0 && tgt == rd32(this.wrapping_add(LATCHED));

        if (flags as u8) == 0 && rd8(this.wrapping_add(RANGE_ON)) != 0 {
            let q = rd32(this.wrapping_add(QUEUE));
            let r = rd32(q.wrapping_add(PROBE_OFF));
            if r != 0 {
                let pslot = rd32(rd32(r).wrapping_add(VT_PROBE));
                let probe: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(pslot as usize);
                if probe(r) != PROBE_SKIP {
                    let g = rd32(this.wrapping_add(GUARD));
                    if g == 0 || rd8(g.wrapping_add(GUARD_FLAG)) == 0 {
                        lf_checker_rt::callee_thiscall!(CAL_LATCH, u32, q, 1);
                    }
                }
            }
        }
        if rd8(this.wrapping_add(RANGE_ON)) != 0 {
            let m = rd32(this.wrapping_add(MATRIX));
            let d0 = sub(f0, rdf(m.wrapping_add(ROW_X)));
            let d1 = sub(f1, rdf(m.wrapping_add(ROW_Y)));
            let d2 = sub(f2, rdf(m.wrapping_add(ROW_Z)));
            let dd = add(add(mul(d1, d1), mul(d0, d0)), mul(d2, d2));
            if dd > f32::from_bits(rd_global(THRESH)) {
                lf_checker_rt::callee_cdecl!(CAL_FAR, u32,);
                let mut arr = [f0, f1, f2];
                lf_checker_rt::callee_cdecl!(CAL_HANDOFF, u32, arr.as_mut_ptr() as u32);
            }
            wr32(
                this.wrapping_add(MARK_RANGE),
                rd32(this.wrapping_add(MARK_RANGE)) | MARK_RANGE_BITS,
            );
        }

        let tgt2 = rd32(this.wrapping_add(TARGET));
        if tgt2 != 0 && tgt2 == rd32(this.wrapping_add(LATCHED)) {
            lf_checker_rt::callee_thiscall!(CAL_RELATCH, u32, this, 1, 1, 1);
        }
        if aligned {
            let aux = rd32(this.wrapping_add(AUX));
            wr32(aux, rd32(aux) & !AUX_BIT);
            lf_checker_rt::callee_thiscall!(CAL_ALIGN, u32, this, 1, 0);
        }
        let slot = rd32(this.wrapping_add(SLOT));
        if slot != 0 {
            lf_checker_rt::callee_thiscall!(CAL_DRAIN, u32, slot, this.wrapping_add(SLOT));
            wr32(this.wrapping_add(SLOT), 0);
        }
        wr32(this.wrapping_add(STATUS), rd32(this.wrapping_add(STATUS)) & !STATUS_BIT);
        wr32(this.wrapping_add(0xad8), 0);
        wr32(this.wrapping_add(0xad4), 0);
        wr32(this.wrapping_add(0xad0), 0);
        wr32(this.wrapping_add(0xae8), 0);
        wr32(this.wrapping_add(0xae4), 0);
        wr32(this.wrapping_add(0xae0), 0);
        wr32(this.wrapping_add(0xaf8), 0);
        wr32(this.wrapping_add(0xaf4), 0);
        wr32(this.wrapping_add(0xaf0), 0);
        {
            let oslot = rd32(rd32(this).wrapping_add(VT_OUT));
            let out: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(oslot as usize);
            let mut arr = [f0, f1, f2];
            out(this, arr.as_mut_ptr() as u32, 1, 0);
        }
        {
            let fslot = rd32(rd32(this).wrapping_add(VT_FLOAT));
            let fl: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(fslot as usize);
            fl(this, rd32(this.wrapping_add(FLOAT)));
        }
        lf_checker_rt::callee_thiscall!(CAL_SYNC, u32, this);
        {
            let oslot = rd32(rd32(this).wrapping_add(VT_ONE));
            let one: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(oslot as usize);
            one(this, 1);
        }
        wr8(this.wrapping_add(SEEN), 1);

        let d38 = rd32(this.wrapping_add(TARGET));
        if d38 != 0 && rd16(d38.wrapping_add(8)) != NO_TARGET {
            let idx = rd16(d38.wrapping_add(8)) as u32;
            let rows = rd32(rd_global(DISPATCH).wrapping_add(DISPATCH_ROWS));
            let ent = rd32(rows.wrapping_add(idx.wrapping_mul(8).wrapping_add(4)));
            if (ent as u8) & 3 == 1 {
                lf_checker_rt::callee_thiscall!(CAL_DISPATCH, u32, this);
            }
        }

        if lf_checker_rt::callee_thiscall!(CAL_SWEEP, u32, this) != 0 {
            let m20 = rd32(this.wrapping_add(MATRIX));
            let a2 = lf_checker_rt::callee_thiscall!(CAL_SWEEP, u32, this);
            lf_checker_rt::callee_thiscall!(CAL_APPLY_A, u32, a2, m20);
            let a3 = lf_checker_rt::callee_thiscall!(CAL_SWEEP, u32, this);
            lf_checker_rt::callee_thiscall!(CAL_APPLY_B, u32, a3, m20);
            let a4 = lf_checker_rt::callee_thiscall!(CAL_SWEEP, u32, this);
            let sslot = rd32(rd32(a4).wrapping_add(VT_STEP));
            let step: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(sslot as usize);
            step(a4);
            // Two more sweep calls: the first one's selector word is tested,
            // the second one's becomes callee L's object.
            let a5 = lf_checker_rt::callee_thiscall!(CAL_SWEEP, u32, this);
            if rd32(a5.wrapping_add(SELECT)) != 0 {
                let a6 = lf_checker_rt::callee_thiscall!(CAL_SWEEP, u32, this);
                lf_checker_rt::callee_thiscall!(CAL_SELECT, u32, rd32(a6.wrapping_add(SELECT)), 0);
            }
        }

        let head = rd32(this.wrapping_add(LATCHED));
        if head != 0 && rd16(head.wrapping_add(8)) != NO_TARGET {
            let st = rd32(this.wrapping_add(HEAD_STATE));
            if st != STATE_A && st != STATE_B {
                let hslot = rd32(rd32(head).wrapping_add(VT_HEAD));
                let head_call: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(hslot as usize);
                head_call(head, head.wrapping_add(HEAD_OFF));
                lf_checker_rt::callee_thiscall!(CAL_SAMPLE, u32, head, rd32(this.wrapping_add(MATRIX)));
                let hslot2 = rd32(rd32(head).wrapping_add(VT_HEAD));
                let head_call2: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(hslot2 as usize);
                head_call2(head, head.wrapping_add(HEAD_OFF));
                lf_checker_rt::callee_thiscall!(CAL_HEAD_SYNC, u32, this);
                let bslot = rd32(rd32(head).wrapping_add(VT_STEP2));
                let bstep: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(bslot as usize);
                bstep(head);
                let bslot2 = rd32(rd32(head).wrapping_add(VT_STEP2));
                let bstep2: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(bslot2 as usize);
                bstep2(head);
                let e = rd32(head.wrapping_add(PROBE_LINK));
                let probe = rd32(e.wrapping_add(PROBE_NEXT));
                let saved = rdf(probe.wrapping_add(PROBE_VAL));
                lf_checker_rt::callee_thiscall!(CAL_PROBE_RUN, u32, probe, 1);
                let current = rdf(probe.wrapping_add(PROBE_VAL));
                let idx = rd16(head.wrapping_add(8)) as u32;
                let disp = rd_global(DISPATCH);
                if saved == current {
                    lf_checker_rt::callee_thiscall!(CAL_EQUAL, u32, disp, idx, 0);
                } else {
                    lf_checker_rt::callee_thiscall!(
                        CAL_UNEQUAL,
                        u32,
                        disp,
                        idx,
                        saved.to_bits(),
                        0
                    );
                }
            }
        }

        wr32(this.wrapping_add(MODE), rd32(this.wrapping_add(MODE)) & !TAIL_BIT);
        lf_checker_rt::callee_thiscall!(
            CAL_NOTIFY,
            u32,
            lf_checker_rt::relocated(SERVICE),
            this
        );
        0
    }
});
