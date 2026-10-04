// original: 0x00a731a0 CTaskComplexPlayerSettingsTask::vf5 (symbols)
/// Double registry check, a conditional action, then the sub-task hook
/// with a fixed middle argument.
///
/// `thiscall`: object in ECX, three stack words (entity, spare, target),
/// callee pops 12. Pushes the selector global and the key at `this+0x20`
/// for the registry lookup (callee 1, one word), maps the answer
/// (callee 2, thiscall) and tests the pair with the first predicate
/// (callee 3, two words: mapped answer, selector); a pass repeats the
/// lookup with the second predicate (callee 4). When the readiness test
/// (callee 5, thiscall on `entity+0x2b0` with 1) passes, the action runs
/// (callee 6, thiscall on `entity+0x2b0` with the entity). The sub-task
/// hook (virtual slot `+0x14`) is skipped when bit 0 of `sub+0xc` is set,
/// otherwise it runs with (entity, 2, target); a zero answer returns 0
/// and a hook that ran latches bit 1. Otherwise the result is 1.
lf_checker_rt::export!(thiscall, rw_00a731a0(this: u32, ent: u32, _x: u32, target: u32) -> u8 {
    unsafe {
        const SELECTOR: u32 = 0x012b4138;
        const KEY_OFF: u32 = 0x20;
        const ENT_OFF: u32 = 0x2b0;
        const SUB_OFF: u32 = 0x8;
        const FLAG_OFF: u32 = 0xc;
        const HOOK_SLOT: u32 = 0x14;
        const FIXED_MID: u32 = 2;
        let sel = (lf_checker_rt::global::<u32>(SELECTOR)).read_unaligned();
        let key = ((this + KEY_OFF) as *const u32).read_unaligned();
        let m1 = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            lf_checker_rt::callee_cdecl!(1, u32, key)
        );
        if lf_checker_rt::callee_cdecl!(3, u32, m1, sel) as u8 != 0 {
            let m2 = lf_checker_rt::callee_thiscall!(
                2,
                u32,
                lf_checker_rt::callee_cdecl!(1, u32, key)
            );
            lf_checker_rt::callee_cdecl!(4, u32, m2, sel);
        }
        if lf_checker_rt::callee_thiscall!(5, u32, ent.wrapping_add(ENT_OFF), 1) as u8 != 0
        {
            lf_checker_rt::callee_thiscall!(6, u32, ent.wrapping_add(ENT_OFF), ent);
        }
        let sub = ((this + SUB_OFF) as *const u32).read_unaligned();
        if ((sub + FLAG_OFF) as *const u8).read() & 1 == 0 {
            let vt = (sub as *const u32).read_unaligned();
            let hook: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(
                    (((vt + HOOK_SLOT) as *const u32).read_unaligned()) as usize,
                );
            if hook(sub, ent, FIXED_MID, target) as u8 == 0 {
                return 0;
            }
            let w = ((sub + FLAG_OFF) as *const u32).read_unaligned();
            ((sub + FLAG_OFF) as *mut u32).write_unaligned(w | 2);
        }
        1
    }
});
