// original: 0x00884c60 stream_mgr_detach (proposed)
/// Detach one slot from the manager's live set under the manager lock.
///
/// Takes the manager lock at `mgr+0x45c` (intercepted callee 1, thiscall:
/// scratch in `ecx`, lock address as the stack argument), scans the 256
/// half-word live set at `mgr+0x21c` for `slot`, and on a hit marks the
/// entry `FREE` (`0xffff`) and decrements the live count at `mgr+0x444`,
/// then releases the lock (intercepted callee 2, thiscall, scratch in
/// `ecx`). The scratch area is two zero words, matching the checker's zero
/// stack fill.
///
/// Return value: the original ends with `(an instruction of the original)` after the intercepted
/// unlock call, and the stub sets all of `eax` to its scripted answer, so the
/// upper bytes are the stub's, not the scan index's: the rewrite returns the
/// stub's answer with its low byte replaced by the found flag.
///
/// Original: thiscall, one stack argument, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00884c60(mgr: u32, slot: u32) -> u32 {
    unsafe {
        const LIVE_SET: u32 = 0x21c;
        const LIVE_COUNT: u32 = 0x444;
        const MGR_LOCK: u32 = 0x45c;
        const SET_SIZE: u32 = 0x100;
        const FREE: u16 = 0xffff;
        const LOCK_CALLEE: u32 = 1;
        const UNLOCK_CALLEE: u32 = 2;
        let mut scratch = [0u32; 2];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOCK_CALLEE,
            u32,
            scratch.as_mut_ptr() as u32,
            mgr.wrapping_add(MGR_LOCK)
        );
        let mut found = SET_SIZE;
        let mut i = 0u32;
        while i < SET_SIZE {
            let v = ((mgr + LIVE_SET + i.wrapping_mul(2)) as *const u16).read_unaligned() as u32;
            if v == slot {
                found = i;
                break;
            }
            i += 1;
        }
        let hit = found < SET_SIZE;
        if hit {
            ((mgr + LIVE_SET + found.wrapping_mul(2)) as *mut u16).write_unaligned(FREE);
            let count = (mgr + LIVE_COUNT) as *mut u32;
            count.write_unaligned(count.read_unaligned().wrapping_sub(1));
        }
        let unlock_answer: u32 =
            lf_checker_rt::callee_thiscall!(UNLOCK_CALLEE, u32, scratch.as_mut_ptr() as u32);
        // The original's trailing `(an instruction of the original)` keeps the stub's upper bytes.
        (unlock_answer & 0xFFFF_FF00) | (hit as u32)
    }
});
