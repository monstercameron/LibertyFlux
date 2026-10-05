// original: 0x009F4200 peds_tasks_register_all (proposed)

/// Register every ped task handler and latch the task tick.
///
/// Straight-line setup code (cdecl, no arguments, no branches): it opens
/// with callee 8, then runs three phases and a tail, threading scripted
/// answers between calls exactly as the original does:
///
/// - Phase one resolves eight handler keys (`KEYS`) through callee 1 with
///   the shared registry word and a constant tag of 2, projects each answer
///   through callee 2, and submits the projection to callee 3.
/// - A separator pair (callees 4 and 5, both with a zero argument) follows;
///   the last phase-one cleanup is merged into the separator's stack
///   adjustment, which is unobservable.
/// - Phase two programs eight handler rows: each takes callee 5's answer
///   plus `ROW_OFF` as its object and one `(key, value)` pair from `ROWS`
///   with three zero arguments through callee 6, then re-arms callee 5.
/// - Phase three repeats the eight keys through callees 1, 2 and 7, this
///   time with only two arguments to callee 1 (the tag word is absent);
///   the last iteration's cleanup is likewise merged into the tail's.
/// - The tail copies the global tick to the latch word and runs callee 9
///   with a zero argument.
///
/// Callee 1 is declared with the two leading arguments: phase one pushes a
/// third constant word that neither side's comparison observes (scratch
/// below the incoming stack pointer), and the rewrite pushes it too so both
/// sides run the identical sequence.
///
/// Returns nothing; the original is void.
lf_checker_rt::export!(cdecl, rw_009F4200() -> u32 {
    unsafe {
        const REGISTRY: u32 = 0x012b_4138;
        const TICK_GLOBAL: u32 = 0x0117_35b4;
        const LATCH: u32 = 0x012b_61d0;
        const ROW_OFF: u32 = 0x2b0;
        const KEYS: [u32; 8] = [3, 5, 7, 0x0a, 0x0c, 0x0e, 0x10, 0x12];
        const ROWS: [(u32, u32); 8] = [
            (3, 1),
            (5, 0x0a),
            (7, 0x78),
            (0x0a, 0x32),
            (0x0c, 0x96),
            (0x0e, 0x64),
            (0x10, 0xc8),
            (0x12, 0x14),
        ];
        const CAL_OPEN: u32 = 8;
        const CAL_RESOLVE: u32 = 1;
        const CAL_PROJECT: u32 = 2;
        const CAL_SUBMIT_A: u32 = 3;
        const CAL_SEP: u32 = 4;
        const CAL_ARM: u32 = 5;
        const CAL_PROGRAM: u32 = 6;
        const CAL_SUBMIT_B: u32 = 7;
        const CAL_CLOSE: u32 = 9;

        #[inline(always)]
        unsafe fn rd_global(file_va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr_global(file_va: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(file_va).write_unaligned(v) }
        }

        lf_checker_rt::callee_cdecl!(CAL_OPEN, u32,);
        for k in KEYS {
            let h = lf_checker_rt::callee_cdecl!(CAL_RESOLVE, u32, k, rd_global(REGISTRY), 2);
            let p = lf_checker_rt::callee_thiscall!(CAL_PROJECT, u32, h);
            lf_checker_rt::callee_cdecl!(CAL_SUBMIT_A, u32, p);
        }
        lf_checker_rt::callee_cdecl!(CAL_SEP, u32, 0);
        let mut arm = lf_checker_rt::callee_cdecl!(CAL_ARM, u32, 0);
        for (y, x) in ROWS {
            lf_checker_rt::callee_thiscall!(
                CAL_PROGRAM,
                u32,
                arm.wrapping_add(ROW_OFF),
                y,
                x,
                0,
                0,
                0
            );
            arm = lf_checker_rt::callee_cdecl!(CAL_ARM, u32, 0);
        }
        for k in KEYS {
            let h = lf_checker_rt::callee_cdecl!(CAL_RESOLVE, u32, k, rd_global(REGISTRY));
            let p = lf_checker_rt::callee_thiscall!(CAL_PROJECT, u32, h);
            lf_checker_rt::callee_cdecl!(CAL_SUBMIT_B, u32, p);
        }
        wr_global(LATCH, rd_global(TICK_GLOBAL));
        lf_checker_rt::callee_cdecl!(CAL_CLOSE, u32, 0);
        0
    }
});
