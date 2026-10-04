// original: 0x009F5470 ACHBLOCKED1C

/// Decide whether an achievement slot is blocked and report the verdict.
///
/// `arg` selects one of 32 slots. Callee 1 first resolves the entry tag for
/// `ENTRY_STR`. Slot 0xFFFFFFFF in `IDX`, a null row in `OBJ_TABLE`, or a
/// zero low byte from callee 2 (thiscall on the row) clears the ready flag;
/// a nonzero low byte from callee 3, a set word at `KILL_FLAG`, a clear
/// ready flag, or a slot above 31 skips the switch and reports the entry
/// tag. Otherwise the slot dispatches through a five-entry table: slots
/// 0, 1 and 22 compare `STAMP_DST` against `STAMP_SRC` (equal returns the
/// stamp at once, skipping the report) and otherwise probe gate 0x13; slot
/// 2 stamps `STAMP_DST` and walks five gate chains; slots 4, 7, 26, 27, 30
/// and 31 walk three; slot 5 probes gate 0x19; every other slot reports at
/// once. Each chain probes its gates in order through callee 4: the first
/// gate with a nonzero low byte advances, an all-zero chain re-resolves the
/// tag for its own string and reports, and a nonzero final gate reports
/// with the entry tag. The report is callee 5 with fourteen arguments; its
/// answer is returned.
///
/// Only low bytes of callee 2, 3 and 4 answers matter. Original: 0x009F5470
/// (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_009f5470(arg: u32) -> u32 {
    unsafe {
        const ENTRY_STR: u32 = 0x0E98_984;
        const STR_S1: u32 = 0x0E98_98C;
        const STR_S2: u32 = 0x0E98_99C;
        const STR_S3: u32 = 0x0E98_9AC;
        const STR_S4: u32 = 0x0E98_9BC;
        const STR_S5: u32 = 0x0E98_9CC;
        const STR_T1: u32 = 0x0E98_9DC;
        const STR_T2: u32 = 0x0E98_9EC;
        const STR_T3: u32 = 0x0E98_9FC;
        const STR_C3: u32 = 0x0E98_A0C;
        const STR_C0: u32 = 0x0E98_A18;
        const NAME_OBJ: u32 = 0x0116_BFF0;
        const NOTIFY_OBJ: u32 = 0x0103_3130;
        const IDX: u32 = 0x0103_6F14;
        const OBJ_TABLE: u32 = 0x011A_8808;
        const KILL_FLAG: u32 = 0x011D_6FD4;
        const STAMP_SRC: u32 = 0x0117_35B4;
        const STAMP_DST: u32 = 0x012B_625C;
        const NO_INDEX: u32 = 0xFFFF_FFFF;
        const MAX_SLOT: u32 = 0x1F;

        #[inline(always)]
        unsafe fn low_set(v: u32) -> bool {
            (v & 0xFF) != 0
        }
        #[inline(always)]
        unsafe fn gate(val: u32) -> bool {
            unsafe { low_set(lf_checker_rt::callee_cdecl!(4, u32, val)) }
        }
        #[inline(always)]
        unsafe fn any_of(vals: &[u32]) -> bool {
            unsafe {
                for &v in vals {
                    if gate(v) {
                        return true;
                    }
                }
                false
            }
        }
        #[inline(always)]
        unsafe fn resolve(name: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_thiscall!(
                    1,
                    u32,
                    lf_checker_rt::relocated(NAME_OBJ),
                    lf_checker_rt::relocated(name)
                )
            }
        }
        #[inline(always)]
        unsafe fn notify(tag: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_thiscall!(
                    5, u32, lf_checker_rt::relocated(NOTIFY_OBJ), tag,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0xFFFF_FFFF
                )
            }
        }

        let mut tag = resolve(ENTRY_STR);
        let idx = (lf_checker_rt::global::<u32>(IDX) as *const u32).read();
        let mut ready = false;
        if idx != NO_INDEX {
            let row = (lf_checker_rt::global::<u32>(OBJ_TABLE) as *const u32).add(idx as usize).read();
            if row != 0 {
                ready = low_set(lf_checker_rt::callee_thiscall!(2, u32, row));
            }
        }
        let forced = low_set(lf_checker_rt::callee_cdecl!(3, u32,));
        let killed = (lf_checker_rt::global::<u32>(KILL_FLAG) as *const u32).read() != 0;
        let mut early: Option<u32> = None;
        if !forced && ready && !killed && arg <= MAX_SLOT {
            match arg {
                0 | 1 | 22 => {
                    let kept = (lf_checker_rt::global::<u32>(STAMP_DST) as *const u32).read();
                    let live = (lf_checker_rt::global::<u32>(STAMP_SRC) as *const u32).read();
                    if kept == live {
                        early = Some(kept);
                    } else if gate(0x13) {
                        // report with the entry tag
                    } else {
                        tag = resolve(STR_C0);
                    }
                }
                2 => {
                    let live = (lf_checker_rt::global::<u32>(STAMP_SRC) as *const u32).read();
                    (lf_checker_rt::global::<u32>(STAMP_DST) as *mut u32).write(live);
                    if !any_of(&[5, 0x0F, 0x0B]) {
                        tag = resolve(STR_S1);
                    } else if !any_of(&[0x0F, 0x0B]) {
                        tag = resolve(STR_S2);
                    } else if !gate(5) {
                        tag = resolve(STR_S3);
                    } else if !gate(0x0F) {
                        tag = resolve(STR_S4);
                    } else if !gate(0x0B) {
                        tag = resolve(STR_S5);
                    } else {
                        // report with the entry tag
                    }
                }
                4 | 7 | 26 | 27 | 30 | 31 => {
                    if !any_of(&[0x0F, 0x0B]) {
                        tag = resolve(STR_T1);
                    } else if !gate(0x0F) {
                        tag = resolve(STR_T2);
                    } else if !gate(0x0B) {
                        tag = resolve(STR_T3);
                    } else {
                        // report with the entry tag
                    }
                }
                5 => {
                    if gate(0x19) {
                        // report with the entry tag
                    } else {
                        tag = resolve(STR_C3);
                    }
                }
                _ => {
                    // report with the entry tag
                }
            }
        }
        match early {
            Some(v) => v,
            None => notify(tag),
        }
    }
});
