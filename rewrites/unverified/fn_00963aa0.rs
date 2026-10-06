// original: 0x00963aa0 perf_counter_snapshot
/// Snapshot the 64-bit performance counter into a global pair.
///
/// Takes no arguments. When the availability probe (no arguments) reports a
/// non-zero low byte, reads the selector at `0x11F7060`: 0 snapshots the
/// counter (no arguments, 64-bit answer) into `0x120F2B0`/`0x120F2B4`, 1
/// into `0x120F2A8`/`0x120F2AC`, anything else stores nothing. Returns the
/// probe answer when it vetoes, the counter's low word after a snapshot, or
/// the selector otherwise.
lf_checker_rt::export!(cdecl, rw_00963aa0() -> u32 {
    unsafe {
        const SEL: u32 = 0x11f7060;
        const PAIR_A: u32 = 0x120f2b0;
        const PAIR_B: u32 = 0x120f2a8;
        let r1: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        if (r1 & 0xff) == 0 {
            return r1;
        }
        let sel = (lf_checker_rt::global::<u32>(SEL) as *const u32).read_unaligned();
        if sel == 0 {
            let t: u64 = lf_checker_rt::callee_cdecl!(2, u64,);
            (lf_checker_rt::global::<u32>(PAIR_A) as *mut u32).write_unaligned(t as u32);
            (lf_checker_rt::global::<u32>(PAIR_A + 4) as *mut u32)
                .write_unaligned((t >> 32) as u32);
            return t as u32;
        }
        if sel == 1 {
            let t: u64 = lf_checker_rt::callee_cdecl!(2, u64,);
            (lf_checker_rt::global::<u32>(PAIR_B) as *mut u32).write_unaligned(t as u32);
            (lf_checker_rt::global::<u32>(PAIR_B + 4) as *mut u32)
                .write_unaligned((t >> 32) as u32);
            return t as u32;
        }
        sel
    }
});
