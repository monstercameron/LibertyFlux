// original: 0x00b75940 ped_task_scan_candidates (proposed)

/// Scan one ped's task candidates and keep the reachable ones.
///
/// `this` points to the ped's task-state block: four floats at `+0x0c`,
/// `+0x10`, `+0x14`, `+0x18` (two planar distances are derived from them)
/// and a selector byte at `+0x28` whose low nibble picks the scan mode.
/// Mode 9 walks candidate list A, every other nibble walks list B with an
/// extra pair of eligibility tests and a per-nibble acceptance rule.
///
/// For each candidate the callee answers supply a row pointer from a global
/// table; the row carries two threshold pairs (`+0x34`/`+0x24`,
/// `+0x30`/`+0x20`), a kind word (`+0x6c`) and a flag word (`+0x94`).
/// A candidate is appended to the scratch set when the ped's distances
/// exceed both thresholds (strictly greater; an unordered NaN comparison
/// appends nothing) and, in mode B, both eligibility answers are zero, the
/// kind is neither 2 nor 4, and the nibble rule accepts it (nibble 0 always
/// accepts; 1/3/7 accept on flag bits 9/1/10; 2/4/8 accept when those bits
/// are clear; 5 accepts kind 1; 6 accepts any kind but 1; anything above 8
/// rejects). The scratch set is then finalised, an extra step runs when it
/// holds more than one entry, the produced value is stored to a global
/// slot and returned.
///
/// Original: 0x00b75940 (thiscall, no stack words; callee 11 is the CRT
/// security-cookie check, which preserves eax/ecx/edx).
const COLL_A: u32 = 0x016dd1b8;
const COLL_B: u32 = 0x016dceb8;
const ROW_TABLE: u32 = 0x01295cd8;
const RESULT_SLOT: u32 = 0x0167cca0;
const THIS_D0X: u32 = 0x0c;
const THIS_D0Y: u32 = 0x10;
const THIS_D1X: u32 = 0x14;
const THIS_D1Y: u32 = 0x18;
const THIS_MODE: u32 = 0x28;
const ROW_LO0: u32 = 0x24;
const ROW_LO1: u32 = 0x20;
const ROW_HI0: u32 = 0x34;
const ROW_HI1: u32 = 0x30;
const ROW_KIND: u32 = 0x6c;
const ROW_FLAGS: u32 = 0x94;
const FLAG_BIT_A: u32 = 9;
const FLAG_BIT_B: u32 = 1;
const FLAG_BIT_C: u32 = 10;
const TEST_ARG_A: u32 = 0x18;
const TEST_ARG_B: u32 = 0x2a;
const MODE_LIST_A: u8 = 9;
const ID_CTOR: u32 = 1;
const ID_COUNT_LIST: u32 = 2;
const ID_COUNT_SET: u32 = 3;
const ID_RESOLVE: u32 = 4;
const ID_APPEND: u32 = 5;
const ID_TEST_A: u32 = 6;
const ID_TEST_B: u32 = 7;
const ID_FINISH: u32 = 8;
const ID_EXTRA: u32 = 9;
const ID_PRODUCE: u32 = 10;
const ID_COOKIE: u32 = 11;

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
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

#[inline(always)]
fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

/// Row pointer for candidate `idx` through the global row table.
#[inline(always)]
unsafe fn row_for(idx: u32) -> u32 {
    unsafe { rd32(lf_checker_rt::relocated(ROW_TABLE).wrapping_add(idx.wrapping_mul(4))) }
}

/// Threshold `hi - lo` read from a candidate row.
#[inline(always)]
unsafe fn threshold(row: u32, hi: u32, lo: u32) -> f32 {
    unsafe { sub(rdf(row.wrapping_add(hi)), rdf(row.wrapping_add(lo))) }
}

/// Nibble acceptance rule for mode B (see doc comment).
#[inline(always)]
fn nibble_accepts(nibble: u8, kind: u32, flags: u32) -> bool {
    {
            0 => true,
            1 => (flags >> FLAG_BIT_A) & 1 != 0,
            2 => (flags >> FLAG_BIT_A) & 1 == 0,
            3 => (flags >> FLAG_BIT_B) & 1 != 0,
            4 => (flags >> FLAG_BIT_B) & 1 == 0,
            5 => kind == 1,
            6 => kind != 1,
            7 => (flags >> FLAG_BIT_C) & 1 != 0,
            8 => (flags >> FLAG_BIT_C) & 1 == 0,
            _ => false,
        }
    }
}

unsafe fn run_00b75940(this: u32, mode_list_a: u8) -> u32 {
    unsafe {
        let mut set = [0u32; 8];
        let set_ptr = set.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(ID_CTOR, u32, set_ptr);
        let dx0 = rdf(this.wrapping_add(THIS_D0X));
        let dy0 = rdf(this.wrapping_add(THIS_D0Y));
        let mut dist0 = add(mul(dx0, dx0), mul(dy0, dy0));
        dist0 = dist0.sqrt();
        let dx1 = rdf(this.wrapping_add(THIS_D1X));
        let dy1 = rdf(this.wrapping_add(THIS_D1Y));
        let mut dist1 = add(mul(dx1, dx1), mul(dy1, dy1));
        dist1 = dist1.sqrt();
        lf_checker_rt::callee_thiscall!(ID_CTOR, u32, set_ptr);
        let nibble = ((this.wrapping_add(THIS_MODE) as *const u8).read() as u8) & 0x0f;
        let append_at = set_ptr.wrapping_add(4);
        if nibble == mode_list_a {
            let mut count = lf_checker_rt::callee_thiscall!(ID_COUNT_LIST, u32, lf_checker_rt::relocated(COLL_A));
            if (count as i32) > 0 {
                let mut i = 0u32;
                loop {
                    let idx = lf_checker_rt::callee_thiscall!(ID_RESOLVE, u32, lf_checker_rt::relocated(COLL_A), i);
                    let row = row_for(idx);
                    if dist0 > threshold(row, ROW_HI0, ROW_LO0) {
                        if dist1 > threshold(row, ROW_HI1, ROW_LO1) {
                            lf_checker_rt::callee_thiscall!(ID_APPEND, u32, append_at, idx);
                        }
                    }
                    i = i.wrapping_add(1);
                    count = lf_checker_rt::callee_thiscall!(ID_COUNT_LIST, u32, lf_checker_rt::relocated(COLL_A));
                    if !((i as i32) < (count as i32)) {
                        break;
                    }
                }
            }
        } else {
            let mut count = lf_checker_rt::callee_thiscall!(ID_COUNT_LIST, u32, lf_checker_rt::relocated(COLL_B));
            if (count as i32) > 0 {
                let mut i = 0u32;
                loop {
                    let idx = lf_checker_rt::callee_thiscall!(ID_RESOLVE, u32, lf_checker_rt::relocated(COLL_B), i);
                    let row = row_for(idx);
                    let probe = f32::from_bits(set[0]);
                    if probe > threshold(row, ROW_HI0, ROW_LO0) {
                        if dist0 > threshold(row, ROW_HI1, ROW_LO1) {
                            let t0 = lf_checker_rt::callee_cdecl!(ID_TEST_A, u32, idx, TEST_ARG_A);
                            if (t0 & 0xff) == 0 {
                                let t1 = lf_checker_rt::callee_cdecl!(ID_TEST_B, u32, idx, TEST_ARG_B);
                                if (t1 & 0xff) == 0 {
                                    let kind = rd32(row.wrapping_add(ROW_KIND));
                                    if kind != 2 && kind != 4 {
                                        let flags = rd32(row.wrapping_add(ROW_FLAGS));
                                        if nibble_accepts(nibble, kind, flags) {
                                            lf_checker_rt::callee_thiscall!(ID_APPEND, u32, append_at, idx);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    i = i.wrapping_add(1);
                    count = lf_checker_rt::callee_thiscall!(ID_COUNT_LIST, u32, lf_checker_rt::relocated(COLL_B));
                    if !((i as i32) < (count as i32)) {
                        break;
                    }
                }
            }
        }
        lf_checker_rt::callee_thiscall!(ID_FINISH, u32, set_ptr);
        let kept = lf_checker_rt::callee_thiscall!(ID_COUNT_SET, u32, set_ptr);
        if (kept as i32) > 1 {
            let prev = (lf_checker_rt::global::<u32>(RESULT_SLOT) as *const u32).read_unaligned();
            let mut d0slot = dist0;
            lf_checker_rt::callee_thiscall!(ID_EXTRA, u32, (&mut d0slot as *mut f32) as u32, prev);
        }
        let mut d1slot = dist1;
        let out = lf_checker_rt::callee_thiscall!(ID_PRODUCE, u32, (&mut d1slot as *mut f32) as u32);
        (lf_checker_rt::global::<u32>(RESULT_SLOT) as *mut u32).write_unaligned(out);
        lf_checker_rt::callee_cdecl!(ID_COOKIE, u32);
        out
    }
}

lf_checker_rt::export!(thiscall, rw_00b75940(this: u32) -> u32 {
    unsafe { run_00b75940(this, MODE_LIST_A) }
});
