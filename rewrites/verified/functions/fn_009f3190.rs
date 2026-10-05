// original: 0x009F3190 ped_task_boot_chain (proposed)

/// Run a ped's task-boot chain: probe the auxiliary task object, refresh the
/// ped's latched task head, and finish through the tail notifier.
///
/// `this` is the ped; `arg1` and `arg2` are small flags (only their low
/// bytes are tested).
///
/// When the tabled flag (`FLAG_TABLED` in `+FLAGS`) is set and the auxiliary
/// object at `+AUX` exists, the boot block runs: callee 1 classifies the
/// pair, then callees 2, 3 and 5 each take a scratch state struct (passed by
/// frame pointer; what they store there is never read back, so only the
/// calls themselves are observed). The auxiliary object then either matches
/// `KIND_DIRECT` or resolves through the global variant table indexed by
/// its sign-extended `+TABLE_INDEX` word, in which case descriptor bit
/// `DESC_BIT` of that entry's `+ENTRY_DESC` word decides; on a match,
/// callee 4 runs with the auxiliary object and `FINAL_STATE`. The block
/// ends by marking the latched head (`+HEAD`) at `MARK_READY`.
///
/// The refresh then runs unconditionally: the status word at `+STATUS` is
/// stored, the head's marker at `MARK_DONE` is set when the low nibble of
/// `+CODE` is at least 2 with `CODE_BIT` set, and the virtual hook at slot
/// `+VT_HOOK` runs when the current target (`+TARGET`) is set, mismatched
/// and `arg1` is non-zero. The head is relatched, `+STATUS` advances, and a
/// global tick is copied to `+TICK`.
///
/// When `arg2` is non-zero with a live, mismatched head, the compare block
/// runs unless skipped by `SKIP_BIT` in `+OPTS`: a probe float is saved,
/// callee 6 runs, the head's slots `+VT_PROBE` and twice `+VT_STEP` run,
/// and callee 7 runs. The saved float is then compared against the live one
/// with ordered-equal semantics (a NaN on either side, which is also how the
/// harness reaches the unequal path, counts as unequal): on equality callee
/// 9 runs with the head index, otherwise callee 8 runs with the head index
/// and the saved float bits.
///
/// The tail always runs afterwards: flag `TAIL_BIT` in `+MODE` is cleared,
/// the head index addresses a row of the global dispatch table, and callee
/// 11 runs when that row's tag equals `TAG_WANT`; callee 12 then runs with
/// the ped and its `+WORKER` block. Finally the finish bit in `+DONE` is
/// cleared, callee 13 runs with five arguments, and the virtual slot
/// `+VT_TAIL` runs with the head plus `TAIL_OFF` and tags 1 and 0.
///
/// Two of the original's stores are not replicated: it spills the probe
/// float into its own incoming `arg1` slot (dead afterwards; the rewrite
/// keeps it in a local, and the stack comparison is off for this reason),
/// and callee 1's call site pushes two extra constant words the callee does
/// not pop (dead scratch on both sides).
///
/// Returns nothing; the original is void (thiscall, two stack arguments).
lf_checker_rt::export!(thiscall, rw_009F3190(this: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x26c;
        const FLAG_TABLED: u8 = 4;
        const AUX: u32 = 0xb30;
        const AUX_KIND: u32 = 0x1304;
        const KIND_DIRECT: u32 = 4;
        const TABLE_INDEX: u32 = 0x2e;
        const VARIANT_TABLE: u32 = 0x0129_5cd8;
        const ENTRY_DESC: u32 = 0x94;
        const DESC_BIT: u32 = 5;
        const FINAL_STATE: u32 = 0x40;
        const HEAD: u32 = 0x7b4;
        const MARK_READY: u32 = 0x2e0;
        const MARK_DONE: u32 = 0x2e1;
        const STATUS: u32 = 0x7b8;
        const CODE: u32 = 0x1e2;
        const CODE_BIT: u32 = 13;
        const TARGET: u32 = 0x38;
        const NO_TARGET: u16 = 0xffff;
        const VT_HOOK: u32 = 0xb0;
        const TICK: u32 = 0x7bc;
        const TICK_GLOBAL: u32 = 0x0117_35b4;
        const OPTS: u32 = 0x29f;
        const SKIP_BIT: u8 = 1;
        const PROBE_ARG: u32 = 0x20;
        const VT_PROBE: u32 = 0x10;
        const VT_STEP: u32 = 0xbc;
        const PROBE_OFF: u32 = 0x0c;
        const PROBE_VAL: u32 = 0x08;
        const HEAD_NEXT: u32 = 0x04;
        const MODE: u32 = 0x24;
        const TAIL_BIT: u32 = 4;
        const DISPATCH: u32 = 0x012b_9c78;
        const DISPATCH_ROWS: u32 = 0x70;
        const ROW_STRIDE: u32 = 8;
        const ROW_TAG: u32 = 4;
        const TAG_WANT: u8 = 1;
        const DISPATCH_OBJ: u32 = 0x012b_9c7c;
        const WORKER: u32 = 0x2b0;
        const DONE: u32 = 0xf4;
        const DONE_BIT: u8 = 2;
        const FINISH: u32 = 0x124;
        const FINISH_ARG: u32 = 0x7c0;
        const VT_TAIL: u32 = 0x04;
        const TAIL_OFF: u32 = 0x10;
        const CAL_CLASSIFY: u32 = 1;
        const CAL_PROBE_A: u32 = 2;
        const CAL_PROBE_B: u32 = 3;
        const CAL_FINAL: u32 = 4;
        const CAL_PROBE_C: u32 = 5;
        const CAL_SAMPLE: u32 = 6;
        const CAL_SYNC: u32 = 7;
        const CAL_UNEQUAL: u32 = 8;
        const CAL_EQUAL: u32 = 9;
        const CAL_DISPATCH: u32 = 11;
        const CAL_WORKER: u32 = 12;
        const CAL_FINISH: u32 = 13;

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

        let aux = rd32(this.wrapping_add(AUX));
        if rd8(this.wrapping_add(FLAGS)) & FLAG_TABLED != 0 && aux != 0 {
            let t = lf_checker_rt::callee_stdcall!(CAL_CLASSIFY, u32, aux, this);
            let mut s1 = 0u32;
            let s1p = (&mut s1 as *mut u32) as u32;
            lf_checker_rt::callee_thiscall!(CAL_PROBE_A, u32, s1p, aux, t);
            let mut s2 = 0u32;
            lf_checker_rt::callee_thiscall!(
                CAL_PROBE_B,
                u32,
                (&mut s2 as *mut u32) as u32,
                this
            );
            let aux2 = rd32(this.wrapping_add(AUX));
            if aux2 != 0 {
                let mut matched = rd32(aux2.wrapping_add(AUX_KIND)) == KIND_DIRECT;
                if !matched {
                    let index = (rd16(aux2.wrapping_add(TABLE_INDEX)) as i16) as i32 as u32;
                    let table = lf_checker_rt::relocated(VARIANT_TABLE);
                    let entry = rd32(table.wrapping_add(index.wrapping_mul(4)));
                    matched = ((rd32(entry.wrapping_add(ENTRY_DESC)) >> DESC_BIT) & 1) != 0;
                }
                if matched {
                    lf_checker_rt::callee_thiscall!(CAL_FINAL, u32, this, aux2, FINAL_STATE);
                }
            }
            let head0 = rd32(this.wrapping_add(HEAD));
            wr8(head0.wrapping_add(MARK_READY), 1);
            let mut s3 = 0u32;
            lf_checker_rt::callee_thiscall!(CAL_PROBE_C, u32, (&mut s3 as *mut u32) as u32);
        }

        let code = rd16(this.wrapping_add(CODE));
        wr32(this.wrapping_add(STATUS), 4);
        if code & 0xf >= 2 && ((code >> CODE_BIT) & 1) != 0 {
            wr8(rd32(this.wrapping_add(HEAD)).wrapping_add(MARK_DONE), 1);
        }

        let cur = rd32(this.wrapping_add(TARGET));
        if cur != 0 && rd16(cur.wrapping_add(8)) != NO_TARGET && (arg1 as u8) != 0 {
            let slot = rd32(rd32(this).wrapping_add(VT_HOOK));
            let hook: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
            hook(this);
        }

        let head = rd32(this.wrapping_add(HEAD));
        wr32(this.wrapping_add(TARGET), head);
        wr32(this.wrapping_add(STATUS), 6);
        wr32(this.wrapping_add(TICK), rd_global(TICK_GLOBAL));
        if (arg2 as u8) != 0 && head != 0 && rd16(head.wrapping_add(8)) != NO_TARGET {
            if rd8(this.wrapping_add(OPTS)) & SKIP_BIT == 0 {
                let e = rd32(head.wrapping_add(HEAD_NEXT));
                let probe = rd32(e.wrapping_add(PROBE_OFF));
                let saved = rdf(probe.wrapping_add(PROBE_VAL));
                lf_checker_rt::callee_stdcall!(CAL_SAMPLE, u32, rd32(this.wrapping_add(PROBE_ARG)));
                let vt = rd32(head);
                let pslot = rd32(vt.wrapping_add(VT_PROBE));
                let probe_call: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(pslot as usize);
                probe_call(head, head.wrapping_add(TAIL_OFF));
                lf_checker_rt::callee_thiscall!(CAL_SYNC, u32, this);
                let step: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(head).wrapping_add(VT_STEP)) as usize);
                step(head);
                let step2: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(head).wrapping_add(VT_STEP)) as usize);
                step2(head);
                let current = rdf(probe.wrapping_add(PROBE_VAL));
                let idx = rd16(head.wrapping_add(8)) as u32;
                let disp = rd_global(DISPATCH);
                if saved == current {
                    lf_checker_rt::callee_thiscall!(CAL_EQUAL, u32, disp, idx, 0);
                } else {
                    lf_checker_rt::callee_thiscall!(CAL_UNEQUAL, u32, disp, idx, saved.to_bits(), 0);
                }
            }
            wr32(this.wrapping_add(MODE), rd32(this.wrapping_add(MODE)) & !TAIL_BIT);
            let idxw = rd16(head.wrapping_add(8)) as u32;
            let rows = rd32(rd_global(DISPATCH).wrapping_add(DISPATCH_ROWS));
            let ent = rd32(rows.wrapping_add(idxw.wrapping_mul(ROW_STRIDE).wrapping_add(ROW_TAG)));
            if (ent as u8) & 3 == TAG_WANT {
                lf_checker_rt::callee_thiscall!(
                    CAL_DISPATCH,
                    u32,
                    rd_global(DISPATCH_OBJ),
                    idxw,
                    0,
                    0
                );
            }
            lf_checker_rt::callee_thiscall!(CAL_WORKER, u32, this.wrapping_add(WORKER), this);
        }

        wr8(this.wrapping_add(DONE), rd8(this.wrapping_add(DONE)) & !DONE_BIT);
        lf_checker_rt::callee_thiscall!(
            CAL_FINISH,
            u32,
            this.wrapping_add(FINISH),
            this.wrapping_add(FINISH_ARG),
            0x15,
            0x15,
            this,
            0
        );
        let tail = rd32(this.wrapping_add(HEAD));
        let tslot = rd32(rd32(this).wrapping_add(VT_TAIL));
        let tail_call: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(tslot as usize);
        tail_call(this, tail.wrapping_add(TAIL_OFF), 1, 0);
        0
    }
});
