// original: 0x00c7fd60 scenario_probe_iterate (proposed) — UNVERIFIED (deferred)

// NOTE: deferred with reason `frame_pointer_args` (plus encrypted bytes): the
// function passes pointers to its own aligned stack frame to five callees, so
// no contract can script the callee answers. The rewrite below mirrors the
// logic with locals and is kept for a future checker or a re-run; it has
// never passed and must not be counted as verified.

/// Probe scenario slot `probe`, iterating the owner list for a live entry.
///
/// Resolves `probe` through `c1`; a non-zero low byte, or a stale generation
/// count (shared `hi - lo <= limit`, all globals), fails at once, writing -1
/// and 0 through `out`. Otherwise an iterator object on the stack walks the
/// owners (`c3` fetches each, `c4` tests it): the first live one wins, writing
/// 1 and the owner through `out`. Returns `out` on every path.
///
/// Original: cdecl, two stack words (plain `ret`).
lf_checker_rt::export!(cdecl, rw_00c7fd60(out: u32, probe: u32) -> u32 {
    unsafe {
        const GEN_HI: u32 = 0x11735b4;
        const GEN_LO: u32 = 0x16f80c0;
        const GEN_LIM: u32 = 0x104b914;
        const RATE: u32 = 0x104b918;
        const C1: u32 = 1;
        const C2: u32 = 2;
        const C3: u32 = 3;
        const C4: u32 = 4;
        const C5: u32 = 5;
        let placed: u32 = lf_checker_rt::callee_cdecl!(C1, u32, probe);
        if (placed & 0xff) != 0 {
            (out as *mut u32).write_unaligned(0xffff_ffff);
            ((out + 4) as *mut u32).write_unaligned(0);
            return out;
        }
        let hi = lf_checker_rt::global::<u32>(GEN_HI).read_unaligned();
        let lo = lf_checker_rt::global::<u32>(GEN_LO).read_unaligned();
        let lim = lf_checker_rt::global::<u32>(GEN_LIM).read_unaligned();
        if hi.wrapping_sub(lo) <= lim {
            (out as *mut u32).write_unaligned(0xffff_ffff);
            ((out + 4) as *mut u32).write_unaligned(0);
            return out;
        }
        lf_checker_rt::global::<u32>(GEN_LO).write_unaligned(hi);
        let rate = lf_checker_rt::global::<f32>(RATE).read_unaligned();
        let mut seed = [0u32; 8];
        let mut iter = [0u32; 8];
        lf_checker_rt::callee_thiscall!(C2, u32, seed.as_mut_ptr() as u32, 0x40u32, 0u32, probe,
            rate.to_bits());
        let mut item: u32 = lf_checker_rt::callee_thiscall!(C3, u32, iter.as_mut_ptr() as u32);
        while item != 0 {
            let live: u32 = lf_checker_rt::callee_thiscall!(C4, u32, item);
            if (live & 0xff) != 0 {
                (out as *mut u32).write_unaligned(1);
                ((out + 4) as *mut u32).write_unaligned(item);
                lf_checker_rt::callee_thiscall!(C5, u32, iter.as_mut_ptr() as u32);
                return out;
            }
            item = lf_checker_rt::callee_thiscall!(C3, u32, iter.as_mut_ptr() as u32);
        }
        lf_checker_rt::callee_thiscall!(C5, u32, iter.as_mut_ptr() as u32);
        (out as *mut u32).write_unaligned(0xffff_ffff);
        ((out + 4) as *mut u32).write_unaligned(0);
        out
    }
});
