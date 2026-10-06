// original: 0x005A8CA0 controls_select_and_reset_slots (proposed)

/// Pick the active control row, run its switch action, then reset all slots.
///
/// The classifier callee maps the mode global to a small row offset `v`.
/// The row table holds 24-byte entries; entry `TABLE_INDEX * 3` points at the
/// row bytes (entry `TABLE_INDEX`, entries 24 bytes apart, emitted as
/// (`TABLE_INDEX` * 3) * 8), and the signed 16-bit switch word at row + `v * 22` + 0x12
/// selects the action: 0 runs the notify/apply pair (notify takes (1,
/// relocated NOTIFY_TAG, which the image relocates like a pointer) — the original pushes a third dead word the callee never
/// reads, which this rewrite omits; apply takes no stack arguments and gets
/// notify's answer in ECX), 1 records the five/ones pattern into the control
/// globals (the last word is 1 exactly when the switch-value global is
/// non-zero), anything else skips straight to the slot loop. Every path ends
/// with a 165-round slot reset like `reset_slots_165` (but passing (0, 1):
/// the pushes run in the opposite order here) and returns the
/// final reset answer. All comparisons on the switch word are exact equality
/// against 0 and 1; the final loop bound is an unsigned less-than.
lf_checker_rt::export!(cdecl, rw_005A8CA0() -> u32 {
    unsafe {
        const MODE: u32 = 0x01160_C0C;
        const TABLE_INDEX: u32 = 0x01160_C40;
        const ROW_TABLE: u32 = 0x019D3_3A0;
        const TABLE_ENTRY: u32 = 24;
        const TABLE_FANOUT: u32 = 3;
        const ROW_STRIDE: u32 = 22;
        const SWITCH_OFF: u32 = 0x12;
        const SWITCH_VALUE: u32 = 0x01160_CC8;
        const FLAG_A: u32 = 0x01160_C78;
        const FLAG_B: u32 = 0x01160_C7C;
        const FLAG_C: u32 = 0x01160_C80;
        const READY_A: u32 = 0x01160_E6C;
        const READY_B: u32 = 0x01160_C8C;
        const READY_C: u32 = 0x01160_C90;
        const READY_D: u32 = 0x01160_C94;
        const READY_E: u32 = 0x01160_C98;
        const READY_F: u32 = 0x01160_C9C;
        const SWITCH_NONZERO: u32 = 0x01160_C88;
        const NOTIFY_TAG: u32 = 0x00F8_901C;
        const SLOT_INDEX: u32 = 0x0103_0BA4;
        const SLOT_COUNT: u32 = 0xA5;
        const CLASSIFY: u32 = 1;
        const NOTIFY: u32 = 2;
        const APPLY: u32 = 3;
        const RESET: u32 = 4;

        let mode = lf_checker_rt::global::<u32>(MODE).read();
        let v = lf_checker_rt::callee_cdecl!(CLASSIFY, u32, mode);
        let idx = lf_checker_rt::global::<u32>(TABLE_INDEX).read();
        let row = ((lf_checker_rt::relocated(ROW_TABLE)
            .wrapping_add(idx.wrapping_mul(TABLE_FANOUT).wrapping_mul(8)))
            as *const u32)
            .read();
        let w = ((row.wrapping_add(v.wrapping_mul(ROW_STRIDE)).wrapping_add(SWITCH_OFF))
            as *const i16)
            .read_unaligned() as i32;
        if w == 0 {
            let answer = lf_checker_rt::callee_stdcall!(
                NOTIFY,
                u32,
                1u32,
                lf_checker_rt::relocated(NOTIFY_TAG)
            );
            lf_checker_rt::callee_thiscall!(APPLY, u32, answer);
        } else if w == 1 {
            let sv = lf_checker_rt::global::<u32>(SWITCH_VALUE).read();
            lf_checker_rt::global::<u32>(FLAG_A).write(5);
            lf_checker_rt::global::<u32>(FLAG_B).write(5);
            lf_checker_rt::global::<u32>(FLAG_C).write(5);
            lf_checker_rt::global::<u32>(READY_A).write(1);
            lf_checker_rt::global::<u32>(READY_B).write(1);
            lf_checker_rt::global::<u32>(READY_C).write(1);
            lf_checker_rt::global::<u32>(READY_D).write(1);
            lf_checker_rt::global::<u32>(READY_E).write(1);
            lf_checker_rt::global::<u32>(READY_F).write(1);
            lf_checker_rt::global::<u32>(SWITCH_NONZERO).write(u32::from(sv != 0));
        }
        let mut last = 0u32;
        for i in 0..SLOT_COUNT {
            lf_checker_rt::global::<u32>(SLOT_INDEX).write(i);
            last = lf_checker_rt::callee_cdecl!(RESET, u32, 0u32, 1u32);
        }
        last
    }
});
