// original: 0x00b4f380 attach_anim_or_fallback (proposed)

/// Attach animation 949 to a ped, else animation 942, else fail.
///
/// The ped's rig (at `ped + 0x224`, adjusted by `+0x44`) is asked for
/// animation 949 (callee 1, thiscall). On success the ped's current task
/// (callee 2, thiscall on `this`) is pushed to the new animation (callee 3,
/// thiscall) and 1 is returned. On failure animation 942 is tried (callee 4,
/// same shape); success attaches through callee 5 and returns 1, failure
/// returns 0.
///
/// Original: 0x00b4f380 (thiscall, one stack word; boolean in AL).
lf_checker_rt::export!(thiscall, rw_00b4f380(this: u32, ped: u32) -> u32 {
    unsafe {
        const LOOKUP_FIRST: u32 = 1;
        const CURRENT_TASK: u32 = 2;
        const ATTACH_FIRST: u32 = 3;
        const LOOKUP_FALLBACK: u32 = 4;
        const ATTACH_FALLBACK: u32 = 5;
        const RIG: u32 = 0x224;
        const RIG_ADJ: u32 = 0x44;
        const ANIM_FIRST: u32 = 0x3b5;
        const ANIM_FALLBACK: u32 = 0x3ae;
        let rig = ((ped + RIG) as *const u32).read_unaligned().wrapping_add(RIG_ADJ);
        let anim = lf_checker_rt::callee_thiscall!(LOOKUP_FIRST, u32, rig, ANIM_FIRST);
        if anim != 0 {
            let task = lf_checker_rt::callee_thiscall!(CURRENT_TASK, u32, this);
            lf_checker_rt::callee_thiscall!(ATTACH_FIRST, u32, anim, task);
            return 1;
        }
        let rig = ((ped + RIG) as *const u32).read_unaligned().wrapping_add(RIG_ADJ);
        let anim = lf_checker_rt::callee_thiscall!(LOOKUP_FALLBACK, u32, rig, ANIM_FALLBACK);
        if anim != 0 {
            let task = lf_checker_rt::callee_thiscall!(CURRENT_TASK, u32, this);
            lf_checker_rt::callee_thiscall!(ATTACH_FALLBACK, u32, anim, task);
            return 1;
        }
        0
    }
});
