// original: 0x00be7ac0 obj_sub_notify_sync (proposed)

/// Probe an object's sub-object, notify or reset it, then sync and re-notify.
///
/// Takes the object pointer. Probes the sub-object at `+0x2b0` with the word
/// at `+0x2c4` (callee 1, thiscall, one word): when the probe answers
/// non-zero, notifies (callee 2, thiscall on the sub-object, two words: the
/// object and 1; its entry `ecx` is the probe's exit value, so the rewrite
/// passes the sub-object address and the contract does not compare `ecx` for
/// this site), else resets (callee 3, thiscall, three words: the object, 0,
/// 0, same `ecx` treatment). Then syncs (callee 4, thiscall on the
/// sub-object, one word: the object) and, when the word at `+0x2c4` is still
/// non-zero, notifies again through a second stub of the same callee (callee
/// 5, with `ecx` compared). No meaningful return value (`ret: none`).
///
/// Original: stdcall, one stack word, the callee pops 4 bytes.
lf_checker_rt::export!(stdcall, rw_00be7ac0(obj: u32) -> u32 {
    unsafe {
        const OFF_SUB: u32 = 0x2b0;
        const OFF_VALUE: u32 = 0x2c4;
        const PROBE: u32 = 1;
        const NOTIFY_DIRTY_ECX: u32 = 2;
        const RESET: u32 = 3;
        const SYNC: u32 = 4;
        const NOTIFY: u32 = 5;

        let sub = obj.wrapping_add(OFF_SUB);
        let value = ((obj + OFF_VALUE) as *const u32).read_unaligned();
        let ok: u32 = lf_checker_rt::callee_thiscall!(PROBE, u32, sub, value);
        if ok as u8 != 0 {
            lf_checker_rt::callee_thiscall!(NOTIFY_DIRTY_ECX, u32, sub, obj, 1);
        } else {
            lf_checker_rt::callee_thiscall!(RESET, u32, sub, obj, 0, 0);
        }
        lf_checker_rt::callee_thiscall!(SYNC, u32, sub, obj);
        if ((obj + OFF_VALUE) as *const u32).read_unaligned() != 0 {
            lf_checker_rt::callee_thiscall!(NOTIFY, u32, sub, obj, 1);
        }
        0
    }
});
