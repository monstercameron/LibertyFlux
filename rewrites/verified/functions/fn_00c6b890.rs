// original: 0x00c6b890 stream_has_pending (proposed)

/// Report whether any live slot in the id table still needs work.
///
/// The wanted id must appear in the halfword table (else 0 at once).
/// Every table entry is then probed with the lock word; entries the
/// probe accepts but the verifier rejects count up, and the answer is
/// 1 when at least one such entry exists. Only the low byte of the
/// answer is defined.
///
/// Original: stdcall with one stack word, two call sites in a loop,
/// reads three globals, al-only return.
lf_checker_rt::export!(stdcall, rw_00c6b890(wanted: u32) -> u32 {
    unsafe {
        const IDS: u32 = 0x0169_E1B6;
        const COUNT: u32 = 0x0169_E3D4;
        const LOCK: u32 = 0x012B_4138;
        const PROBE: u32 = 1;
        const VERIFY: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }

        let ids = lf_checker_rt::relocated(IDS);
        let n = rd32(lf_checker_rt::relocated(COUNT));
        let mut found = false;
        let mut i = 0u32;
        while (i as i32) < (n as i32) {
            if rd16(ids.wrapping_add(i.wrapping_mul(2))) == wanted {
                found = true;
                break;
            }
            i = i.wrapping_add(1);
        }
        if !found {
            return 0;
        }
        let lock = rd32(lf_checker_rt::relocated(LOCK));
        let mut pending = 0u32;
        let mut j = 0u32;
        while (j as i32) < (n as i32) {
            let e = rd16(ids.wrapping_add(j.wrapping_mul(2)));
            let p: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32, e, lock);
            if (p & 0xFF) != 0 {
                let v: u32 = lf_checker_rt::callee_cdecl!(VERIFY, u32, e, lock);
                if (v & 0xFF) == 0 {
                    pending = pending.wrapping_add(1);
                }
            }
            j = j.wrapping_add(1);
        }
        ((pending as i32) >= 1) as u32
    }
});
