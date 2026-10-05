// original: 0x009F3CB0 ped_task_state_sync (proposed)

/// Refresh a ped's task-driven visual state and pick its next task variant.
///
/// `this` is the ped object. The refresh runs in four stages, each guarded
/// by the previous one's outcome:
///
/// 1. Gate: ask the auxiliary object at `+AUX` (callee 1) whether state
///    `0x80000000` is active; when it answers zero the rest of the scan is
///    skipped and only stage 3 runs.
/// 2. Chain scan: follow the task chain whose head is at `+CHAIN_HEAD`
///    through `+CHAIN_NEXT` links. A node whose kind word (`+NODE_KIND`)
///    is `KIND_STOP` ends the scan early. (Each iteration recomputes the
///    same priority value twice and compares it against itself, so the
///    below/above branches on it are dead; only the stop-kind test and the
///    link walk matter.)
/// 3. Pose copy: when the scan ran to the end and the ped's marker bytes
///    (`+MARK_A` clear, `+MARK_B` set), counter (`+COUNTER` non-zero) and
///    variant (`+VARIANT` equal to `VARIANT_SPECIAL`) all agree, copy the
///    four staging floats from the first global slot block into `+POSE`;
///    otherwise run the virtual refresh hook (slot `+VT_REFRESH`) and copy
///    the second global slot block instead.
/// 4. Variant pick: when the flag bit `FLAG_TABLED` is set in `+FLAGS`,
///    index the global variant table by the sign-extended word at
///    `+TABLE_INDEX`, read that entry's descriptor word at `+ENTRY_DESC`
///    and store `VARIANT_A` or `VARIANT_B` into `+VARIANT` depending on
///    descriptor bit `DESC_BIT`.
///
/// Finally, unless the ped's current target (`+TARGET`) is already its
/// latched one (`+LATCHED`), confirm with callee 2 and the limit/flag words
/// (`+LIMIT` above `LIMIT_MAX`, or bit `FLAG_BIT` of `+MODE` set); when
/// confirmed, ask the auxiliary object about state `AUX_STATE_FINAL` and,
/// on a non-zero answer, hand that answer to callee 3 with the weight
/// `FINAL_WEIGHT`.
///
/// Returns nothing; the original is void (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_009F3CB0(this: u32) -> u32 {
    unsafe {
        const AUX: u32 = 0x78;
        const GATE_STATE: u32 = 0x8000_0000;
        const CHAIN_HEAD: u32 = 0x224;
        const CHAIN_PTR: u32 = 0x2e0;
        const NODE_KIND: u32 = 0x04;
        const NODE_NEXT: u32 = 0x0c;
        const KIND_STOP: u32 = 0x410;
        const MARK_A: u32 = 0x218;
        const MARK_B: u32 = 0x219;
        const COUNTER: u32 = 0x228;
        const VARIANT: u32 = 0xb98;
        const VARIANT_SPECIAL: u32 = 0x47;
        const VARIANT_A: u32 = 0x45;
        const VARIANT_B: u32 = 0x46;
        const POSE: u32 = 0xbd0;
        const GLOBALS_A: u32 = 0x0110_db10;
        const GLOBALS_B: u32 = 0x0105_0d30;
        const VT_REFRESH: u32 = 0xd0;
        const FLAGS: u32 = 0x26c;
        const FLAG_TABLED: u8 = 4;
        const TABLE_INDEX: u32 = 0x2e;
        const VARIANT_TABLE: u32 = 0x0129_5cd8;
        const ENTRY_DESC: u32 = 0x120;
        const DESC_BIT: u32 = 2;
        const TARGET: u32 = 0x38;
        const LATCHED: u32 = 0x7b4;
        const LIMIT: u32 = 0xb80;
        const LIMIT_MAX: u32 = 2;
        const MODE: u32 = 0x2a0;
        const MODE_SHIFT: u32 = 15;
        const AUX_STATE_FINAL: u32 = 0x0008_0000;
        const FINAL_WEIGHT: u32 = 0xc100_0000;
        const CAL_GATE: u32 = 1;
        const CAL_CONFIRM: u32 = 2;
        const CAL_APPLY: u32 = 3;

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
        unsafe fn rd_global(file_va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn copy_pose(dst: u32, src_global: u32) {
            unsafe {
                wr32(dst, rd_global(src_global));
                wr32(dst.wrapping_add(4), rd_global(src_global.wrapping_add(4)));
                wr32(dst.wrapping_add(8), rd_global(src_global.wrapping_add(8)));
                wr32(dst.wrapping_add(12), rd_global(src_global.wrapping_add(12)));
            }
        }

        let aux = rd32(this.wrapping_add(AUX));
        if lf_checker_rt::callee_thiscall!(CAL_GATE, u32, aux, GATE_STATE, 1) != 0 {
            let mid = rd32(this.wrapping_add(CHAIN_HEAD));
            let mut node = rd32(mid.wrapping_add(CHAIN_PTR));
            let mut scanned = node == 0;
            if !scanned {
                loop {
                    if rd32(node.wrapping_add(NODE_KIND)) == KIND_STOP {
                        break;
                    }
                    node = rd32(node.wrapping_add(NODE_NEXT));
                    if node == 0 {
                        scanned = true;
                        break;
                    }
                }
            }
            if scanned {
                if rd8(this.wrapping_add(MARK_A)) == 0
                    && rd8(this.wrapping_add(MARK_B)) != 0
                    && rd32(this.wrapping_add(COUNTER)) != 0
                    && rd32(this.wrapping_add(VARIANT)) == VARIANT_SPECIAL
                {
                    copy_pose(this.wrapping_add(POSE), GLOBALS_A);
                } else {
                    let slot = rd32(rd32(this).wrapping_add(VT_REFRESH));
                    let hook: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(slot as usize);
                    hook(this);
                    copy_pose(this.wrapping_add(POSE), GLOBALS_B);
                }
            }
        }

        if rd8(this.wrapping_add(FLAGS)) & FLAG_TABLED != 0 {
            let index = (rd16(this.wrapping_add(TABLE_INDEX)) as i16) as i32 as u32;
            let table = lf_checker_rt::relocated(VARIANT_TABLE);
            let entry = rd32(table.wrapping_add(index.wrapping_mul(4)));
            let desc = rd32(entry.wrapping_add(ENTRY_DESC));
            wr32(
                this.wrapping_add(VARIANT),
                if desc & DESC_BIT != 0 { VARIANT_A } else { VARIANT_B },
            );
        }

        let target = rd32(this.wrapping_add(TARGET));
        let mut confirmed = target != 0 && target == rd32(this.wrapping_add(LATCHED));
        if !confirmed {
            let ok = lf_checker_rt::callee_thiscall!(CAL_CONFIRM, u32, this);
            confirmed = (ok as u8) != 0
                || rd32(this.wrapping_add(LIMIT)) > LIMIT_MAX
                || ((rd32(this.wrapping_add(MODE)) >> MODE_SHIFT) & 1) != 0;
        }
        if confirmed {
            let handle = lf_checker_rt::callee_thiscall!(CAL_GATE, u32, aux, AUX_STATE_FINAL, 1);
            if handle != 0 {
                lf_checker_rt::callee_thiscall!(CAL_APPLY, u32, handle, FINAL_WEIGHT);
            }
        }
        0
    }
});
