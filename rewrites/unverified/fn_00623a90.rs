// original: 0x00623a90 net_filter_and_drop_gamers (proposed)

/// Drop gamers that left the session: filter new candidates against the
/// known array, task the new ones, then probe every known entry.
///
/// `this` is the session object (`STATE` at `+0x50` must read 2 or 3, the
/// session id pair at `+ID0`/`+ID1` must not both be -1). `array` points to
/// `count` known 8-byte gamer records (two words each). No return value.
///
/// The session predicate runs first; a nonzero answer ends the call. Then a
/// lookup fills a scratch buffer with candidate records and returns how many
/// (`n`); each candidate already present in the known array is dropped and
/// the survivors are compacted to the front of the buffer. When at least one
/// survives, a drop-gamers task object is created through the task container
/// ( address `TASK_CTN`, or null when the feature flag byte `FEAT_FLAG` is
/// clear); a null object skips the rest. The task is submitted with the
/// session ids, the compacted records and the object, and one of two
/// completion hooks runs depending on the submission answer.
///
/// Finally every known record is probed: a per-record check first, and for
/// records it rejects, a lookup by session id. A lookup hit carries a use
/// count; a positive count is reported with the record's words through the
/// report hook. Original: 0x00623a90 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00623a90(this: u32, array: u32, count: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x50;
        const ID0: u32 = 0xc30;
        const ID1: u32 = 0xc34;
        const FEAT_FLAG: u32 = 0x18b8309;
        const TASK_CTN: u32 = 0x19f3a10;
        const LOOKUP_TABLE: u32 = 0x6bcee0;
        const GSTATE: u32 = 0x540;
        const GSTATE1: u32 = 0x544;
        const REPORTER: u32 = 0x24;
        const TASK_INFO: u32 = 0x3a0;
        const PRED: u32 = 1;
        const LOOKUP: u32 = 2;
        const FILL: u32 = 3;
        const TASK_NEW: u32 = 4;
        const TASK_SUBMIT: u32 = 5;
        const HOOK_OK: u32 = 6;
        const HOOK_FAIL: u32 = 7;
        const PROBE: u32 = 8;
        const FIND: u32 = 9;
        const REPORT: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        if count == 0 {
            return 0;
        }
        let inhib: u32 = lf_checker_rt::callee_thiscall!(PRED, u32, this);
        if inhib & 0xff != 0 {
            return 0;
        }
        let state = rd32(this.wrapping_add(STATE));
        if state < 2 || state > 3 {
            return 0;
        }
        let id0 = rd32(this.wrapping_add(ID0));
        let id1 = rd32(this.wrapping_add(ID1));
        if id0 == 0xffff_ffff && id1 == 0xffff_ffff {
            return 0;
        }
        let mut buf = [0u32; 6];
        lf_checker_rt::callee_stdcall!(
            LOOKUP, u32, (&mut buf[0] as *mut u32) as u32, 8, 0x20,
            lf_checker_rt::relocated(LOOKUP_TABLE));
        let n = lf_checker_rt::callee_thiscall!(
            FILL, u32, this, (&mut buf[0] as *mut u32) as u32, id0) as i32;
        // Compact the candidates not already known, in place.
        let mut fresh = 0u32;
        if n > 0 {
            let mut w = 0usize;
            for i in 0..n as usize {
                let (a, b) = (buf[i * 2], buf[i * 2 + 1]);
                let mut known = false;
                for k in 0..count as usize {
                    if rd32(array.wrapping_add((k * 8) as u32)) == a
                        && rd32(array.wrapping_add((k * 8 + 4) as u32)) == b
                    {
                        known = true;
                        break;
                    }
                }
                if !known {
                    buf[w * 2] = a;
                    buf[w * 2 + 1] = b;
                    w += 1;
                }
            }
            fresh = w as u32;
        }
        if n > 0 && fresh > 0 {
            let flag = (lf_checker_rt::global::<u8>(FEAT_FLAG) as *const u8).read();
            let ctn = if flag == 0 { 0 } else { lf_checker_rt::relocated(TASK_CTN) };
            let mut task: u32 = 0;
            lf_checker_rt::callee_thiscall!(TASK_NEW, u32, ctn, (&mut task as *mut u32) as u32);
            if task != 0 {
                let ok: u32 = lf_checker_rt::callee_fastcall!(
                    TASK_SUBMIT, u32, 0, this, id0, id1,
                    (&mut buf[0] as *mut u32) as u32, id0, task,
                    task.wrapping_add(TASK_INFO));
                let flag2 = (lf_checker_rt::global::<u8>(FEAT_FLAG) as *const u8).read();
                let ctn2 =
                    if flag2 == 0 { 0 } else { lf_checker_rt::relocated(TASK_CTN) };
                if ok & 0xff != 0 {
                    lf_checker_rt::callee_thiscall!(HOOK_OK, u32, ctn2, 0, task);
                } else {
                    lf_checker_rt::callee_thiscall!(HOOK_FAIL, u32, ctn2, task);
                }
            }
        }
        // Probe every known record.
        let mut cursor = array;
        for _ in 0..count as usize {
            let seen: u32 = lf_checker_rt::callee_thiscall!(PROBE, u32, this, cursor);
            if seen == 0 {
                let _g0 = rd32(this.wrapping_add(GSTATE));
                let _g1 = rd32(this.wrapping_add(GSTATE1));
                let hit: u32 =
                    lf_checker_rt::callee_thiscall!(FIND, u32, this, id0, id1);
                if hit != 0 {
                    let uses = rd32(hit) as i32;
                    if uses > 0 {
                        let w0 = rd32(cursor);
                        let w1 = rd32(cursor.wrapping_add(4));
                        let _ = (w0, w1);
                        let reporter = rd32(this.wrapping_add(REPORTER));
                        let saved = this;
                        lf_checker_rt::callee_thiscall!(
                            REPORT, u32, reporter, uses as u32,
                            (&saved as *const u32) as u32, 0, 0);
                    }
                }
            }
            cursor = cursor.wrapping_add(8);
        }
        0
    }
});
