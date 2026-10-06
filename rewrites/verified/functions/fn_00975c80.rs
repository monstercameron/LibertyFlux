// original: 0x00975c80 audGtaOcclusionGroupManager::vf1

/// Refresh a rotating slice of the occlusion registry.
///
/// `count` entries of the table at `array` are visited starting from a
/// global round-robin cursor (index (cursor + i) % count, unsigned divide;
/// a zero count skips the loop). A set slot whose probe callee answers
/// nonzero is refreshed through two worker callees with `aux`. Two globals
/// are cleared and a maximum is maintained (unsigned), all at 0x121F680.
/// The loop bound is compared unsigned. Returns cursor + 1 in EAX.
/// Original: 0x00975C80 (stdcall, three stack words).
lf_checker_rt::export!(stdcall, rw_00975c80(array: u32, count: u32, aux: u32) -> u32 {
    unsafe {
        const GBASE: u32 = 0x121F680;
        const PROBE_OBJ: u32 = 0x115DEF0;
        const PROBE: u32 = 1;
        const WORK1: u32 = 2;
        const WORK2: u32 = 3;
        let g = lf_checker_rt::global::<u32>(GBASE.wrapping_add(0x10)).read_unaligned();
        lf_checker_rt::global::<u32>(GBASE).write_unaligned(0);
        lf_checker_rt::global::<u32>(GBASE.wrapping_add(8)).write_unaligned(0);
        let stored = g.wrapping_add(1);
        if count != 0 {
            let mut i: u32 = 0;
            while i < count {
                let idx = stored.wrapping_add(i) % count;
                let slot = (array.wrapping_add(idx * 4)) as *const u32;
                let v = slot.read_unaligned();
                if v != 0 {
                    let ok: u32 = lf_checker_rt::callee_thiscall!(
                        PROBE,
                        u32,
                        lf_checker_rt::relocated(PROBE_OBJ)
                    );
                    if ok & 0xFF != 0 {
                        let e = slot.read_unaligned();
                        lf_checker_rt::callee_thiscall!(WORK1, u32, e, aux, 0);
                        let e = slot.read_unaligned();
                        lf_checker_rt::callee_thiscall!(WORK2, u32, e, aux);
                    }
                }
                i += 1;
            }
        }
        let m = lf_checker_rt::global::<u32>(GBASE.wrapping_add(4)).read_unaligned();
        let cur = lf_checker_rt::global::<u32>(GBASE.wrapping_add(8)).read_unaligned();
        let m = if cur > m { cur } else { m };
        lf_checker_rt::global::<u32>(GBASE.wrapping_add(4)).write_unaligned(m);
        stored
    }
});
