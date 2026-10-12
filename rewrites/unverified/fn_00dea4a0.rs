// original: 0x00DEA4A0 UIMontageEditorLayout::vf108

/// Refresh the six child objects in layout order. For each child, call its
/// virtual initialization slot and then its state slot with zero; the third
/// child pointer is reused for a later refresh.
/// Finally call the frame helper on the last child with argument one and
/// return its result. This `thiscall` method has no stack arguments.
lf_checker_rt::export!(thiscall, rw_00DEA4A0(this: u32) -> u32 {
    const FIELD_0: u32 = 0x1E0;
    const FIELD_1: u32 = 0x1E4;
    const FIELD_2: u32 = 0x1E8;
    const FIELD_3: u32 = 0x1F0;
    const FIELD_4: u32 = 0x1F4;
    const FIELD_5: u32 = 0x220;

    let child_0 = unsafe { ((this + FIELD_0) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(1, u32, child_0);
    let _ = lf_checker_rt::callee_thiscall!(2, u32, child_0, 0);
    let child_1 = unsafe { ((this + FIELD_1) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(1, u32, child_1);
    let _ = lf_checker_rt::callee_thiscall!(2, u32, child_1, 0);
    let child_2 = unsafe { ((this + FIELD_2) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(1, u32, child_2);
    let _ = lf_checker_rt::callee_thiscall!(2, u32, child_2, 0);
    let child_3 = unsafe { ((this + FIELD_3) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(1, u32, child_3);
    let _ = lf_checker_rt::callee_thiscall!(2, u32, child_3, 0);
    let child_4 = unsafe { ((this + FIELD_4) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(1, u32, child_4);
    let _ = lf_checker_rt::callee_thiscall!(2, u32, child_4, 0);
    let child_5 = unsafe { ((this + FIELD_2) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(1, u32, child_5);
    let _ = lf_checker_rt::callee_thiscall!(2, u32, child_5, 0);
    let last_child = unsafe { ((this + FIELD_5) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(1, u32, last_child);
    let _ = lf_checker_rt::callee_thiscall!(2, u32, last_child, 0);
    lf_checker_rt::callee_thiscall!(3, u32, last_child, 1)
});
