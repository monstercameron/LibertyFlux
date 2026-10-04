// original: 0x00a6ec80 task_gate_by_meter_and_kind (proposed)

/// Gate deciding whether the task `arg2` may run against target `arg1`:
/// a chain of eligibility tests ending in a meter comparison and a report.
///
/// Tests in order, each returning null on failure: when the mode global at
/// `MODE_GLOBAL` equals `FULL_MODE`, the kind nibble at `arg2 + KIND` must
/// be below 2. The readiness probe (callee 1, one word: 1) must answer
/// non-zero, else the fallback probe (callee 2) must. `arg1` must be
/// non-null. The tag bytes at `arg1 + TAG0/TAG1` xored with the key byte at
/// `arg1 + TAGKEY` must clear `0x7f` and set it respectively (a scrambled
/// two-byte magic). The meter at `this + METER` must read at most 0 (NaN
/// fails). The state word at `arg2 + STATE` must differ from `BUSY_STATE`.
///
/// Then the tick counter (callee 3) is scaled by the scale routine
/// (callee 4), converted to float and divided by the `PER_SECOND` constant.
/// Its absolute value must exceed the `MIN_RATE` constant; the sign (1 when
/// positive, else 0) is reported with the manager in ecx to the report
/// routine (callee 6) after fetching the manager (callee 5), whose null
/// answer also fails. Returns the report call's result.
///
/// Original: thiscall, two stack words (`arg1`, `arg2`), callee pops 8.
/// Float order is the original's: convert, divide, absolute value, compare.
lf_checker_rt::export!(thiscall, rw_00a6ec80(this: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const MODE_GLOBAL: u32 = 0x011d_6fd4;
        const FULL_MODE: u32 = 2;
        const KIND: u32 = 0x1e2;
        const TAGKEY: u32 = 0x26bc;
        const TAG0: u32 = 0x26be;
        const TAG1: u32 = 0x26bf;
        const TAG_LIM: u8 = 0x7f;
        const METER: u32 = 0x7c;
        const STATE: u32 = 0x2b0;
        const BUSY_STATE: u32 = 6;
        const PER_SECOND: u32 = 0x0103_ce8c;
        const MIN_RATE: u32 = 0x00fe_87d0;
        const MANAGER_ANCHOR: u32 = 0x0167_e2a0;
        const READY: u32 = 1;
        const READY_FALLBACK: u32 = 2;
        const TICK: u32 = 3;
        const SCALE: u32 = 4;
        const GET_MANAGER: u32 = 5;
        const REPORT: u32 = 6;

        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        if (lf_checker_rt::relocated(MODE_GLOBAL) as *const u32).read_unaligned() == FULL_MODE {
            let kind = ((arg2 as *const u8).wrapping_byte_offset(KIND as isize)).read() & 0x0f;
            if kind >= 2 {
                return 0;
            }
        }
        let ready: u32 = lf_checker_rt::callee_cdecl!(READY, u32, 1);
        if (ready as u8) == 0 {
            let back: u32 = lf_checker_rt::callee_cdecl!(READY_FALLBACK, u32,);
            if (back as u8) == 0 {
                return 0;
            }
        }
        if arg1 == 0 {
            return 0;
        }
        let key = ((arg1 as *const u8).wrapping_byte_offset(TAGKEY as isize)).read();
        let tag0 = ((arg1 as *const u8).wrapping_byte_offset(TAG0 as isize)).read() ^ key;
        if tag0 <= TAG_LIM {
            return 0;
        }
        let tag1 = ((arg1 as *const u8).wrapping_byte_offset(TAG1 as isize)).read() ^ key;
        if tag1 > TAG_LIM {
            return 0;
        }
        let meter = f32::from_bits(
            ((this as *const u32).wrapping_byte_offset(METER as isize)).read_unaligned());
        if !(meter <= 0.0) {
            return 0;
        }
        let state = ((arg2 as *const u32).wrapping_byte_offset(STATE as isize)).read_unaligned();
        if state == BUSY_STATE {
            return 0;
        }
        let tick: u32 = lf_checker_rt::callee_cdecl!(TICK, u32,);
        let scaled: u32 = lf_checker_rt::callee_cdecl!(SCALE, u32, tick);
        let per_second = f32::from_bits(
            (lf_checker_rt::relocated(PER_SECOND) as *const u32).read_unaligned());
        let rate = div(scaled as i32 as f32, per_second);
        let magnitude = if rate < 0.0 { -rate } else { rate };
        let floor = f32::from_bits(
            (lf_checker_rt::relocated(MIN_RATE) as *const u32).read_unaligned());
        if !(magnitude > floor) {
            return 0;
        }
        let sign = if rate > 0.0 { 1u32 } else { 0u32 };
        let anchor =
            (lf_checker_rt::relocated(MANAGER_ANCHOR) as *const u32).read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
        if mgr == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(REPORT, u32, mgr, sign)
    }
});
