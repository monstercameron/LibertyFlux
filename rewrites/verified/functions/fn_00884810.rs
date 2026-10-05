// original: 0x00884810 stream_mgr_attach (proposed)
/// Attach one slot to the manager's live or pending set under the manager lock.
///
/// Takes the manager lock at `mgr+0x45c` (intercepted callee 1, thiscall:
/// scratch in `ecx`, lock address as the stack argument) and resolves `slot`
/// to an object through the pool lookup (intercepted callee 2, cdecl, one
/// argument). A null object fails. Otherwise the object's flag byte at
/// `+0x14` selects the set: flag bit 0 set scans the 256 half-word live set
/// at `mgr+0x21c` for a `FREE` (`0xffff`) entry and stores the low word of
/// `slot` there, bumping the live count at `mgr+0x444` (failing when the
/// count is already `0x100` or no entry is free); flag bit 0 clear appends
/// the low word at the pending count in `mgr+0x440` into the table at
/// `mgr+0x04` and bumps the count (failing when it is already `0x100`). The
/// lock is released on every path (intercepted callee 3, thiscall, scratch
/// in `ecx`); the scratch area is two zero words, matching the checker's
/// zero stack fill.
///
/// Return value: the original ends with `(an instruction of the original)` after the intercepted
/// unlock call, and the stub sets all of `eax` to its scripted answer, so the
/// upper bytes are the stub's: the rewrite returns the stub's answer with its
/// low byte replaced by the success flag.
///
/// Original: thiscall, one stack argument, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00884810(mgr: u32, slot: u32) -> u32 {
    unsafe {
        const PENDING_TABLE: u32 = 0x04;
        const LIVE_SET: u32 = 0x21c;
        const PENDING_COUNT: u32 = 0x440;
        const LIVE_COUNT: u32 = 0x444;
        const MGR_LOCK: u32 = 0x45c;
        const FLAG_BYTE: u32 = 0x14;
        const SET_SIZE: u32 = 0x100;
        const FREE: u16 = 0xffff;
        const LOCK_CALLEE: u32 = 1;
        const LOOKUP_CALLEE: u32 = 2;
        const UNLOCK_CALLEE: u32 = 3;
        let mut scratch = [0u32; 2];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOCK_CALLEE,
            u32,
            scratch.as_mut_ptr() as u32,
            mgr.wrapping_add(MGR_LOCK)
        );
        let obj: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, slot);
        let mut ok = false;
        if obj != 0 {
            let flags = ((obj + FLAG_BYTE) as *const u8).read();
            if flags & 1 != 0 {
                let live = ((mgr + LIVE_COUNT) as *const u32).read_unaligned();
                if live < SET_SIZE {
                    let mut found = SET_SIZE;
                    let mut i = 0u32;
                    while i < SET_SIZE {
                        let v = ((mgr + LIVE_SET + i.wrapping_mul(2)) as *const u16)
                            .read_unaligned();
                        if v == FREE {
                            found = i;
                            break;
                        }
                        i += 1;
                    }
                    if found < SET_SIZE {
                        ((mgr + LIVE_SET + found.wrapping_mul(2)) as *mut u16)
                            .write_unaligned(slot as u16);
                        let count = (mgr + LIVE_COUNT) as *mut u32;
                        count.write_unaligned(live.wrapping_add(1));
                        ok = true;
                    }
                }
            } else {
                let pending = ((mgr + PENDING_COUNT) as *const u32).read_unaligned();
                if pending < SET_SIZE {
                    let count = (mgr + PENDING_COUNT) as *mut u32;
                    count.write_unaligned(pending.wrapping_add(1));
                    ((mgr + PENDING_TABLE + pending.wrapping_mul(2)) as *mut u16)
                        .write_unaligned(slot as u16);
                    ok = true;
                }
            }
        }
        let unlock_answer: u32 =
            lf_checker_rt::callee_thiscall!(UNLOCK_CALLEE, u32, scratch.as_mut_ptr() as u32);
        // The original's trailing `(an instruction of the original)` keeps the stub's upper bytes.
        (unlock_answer & 0xFFFF_FF00) | (ok as u32)
    }
});
