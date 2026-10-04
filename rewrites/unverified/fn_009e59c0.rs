// original: 0x009e59c0 ped_job_dispatch (proposed)

/// Dispatch a ped job by probe, flag and index pair, then finish.
///
/// Probes (`thiscall` on this, no stack words): a zero low byte skips
/// to the tail. Otherwise runs the one-branch (`cdecl`
/// `(this, 0, 0, 1)`) when the argument's low byte is set, else the
/// zero-branch (`cdecl` `(this, 0, 0)`); then, unless either index at
/// `this + 0xe48`/`+ 0xe4c` is -1, the full runner (`cdecl`
/// `(this, 0, i1, i2, -1, 0, 0)`). The tail clears bit 15 of
/// `this + 0x264`, zeroes `this + 0xe40` and calls finish (`thiscall`
/// on this with `(-1, -1)`), whose answer is returned. `thiscall`, one
/// stack word.
lf_checker_rt::export!(thiscall, rw_009e59c0(this: u32, a1: u32) -> u32 {
    unsafe {
        const IDX1: u32 = 0xe48;
        const IDX2: u32 = 0xe4c;
        const SLOT: u32 = 0xe40;
        const MODE: u32 = 0x264;
        const PROBE: u32 = 1;
        const RUN_ONE: u32 = 2;
        const RUN_ZERO: u32 = 3;
        const RUN_FULL: u32 = 4;
        const FINISH: u32 = 5;
        let probe = lf_checker_rt::callee_thiscall!(PROBE, u32, this);
        if (probe & 0xff) != 0 {
            if (a1 & 0xff) != 0 {
                lf_checker_rt::callee_cdecl!(RUN_ONE, u32, this, 0u32, 0u32, 1u32);
            } else {
                lf_checker_rt::callee_cdecl!(RUN_ZERO, u32, this, 0u32, 0u32);
            }
            let i1 = ((this + IDX1) as *const u32).read_unaligned();
            if i1 != 0xffffffff {
                let i2 = ((this + IDX2) as *const u32).read_unaligned();
                if i2 != 0xffffffff {
                    lf_checker_rt::callee_cdecl!(
                        RUN_FULL, u32, this, 0u32, i1, i2, 0xffffffff, 0u32, 0u32
                    );
                }
            }
        }
        let m = ((this + MODE) as *const u32).read_unaligned();
        ((this + MODE) as *mut u32).write_unaligned(m & 0xffff7fff);
        ((this + SLOT) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(FINISH, u32, this, 0xffffffff, 0xffffffff)
    }
});
