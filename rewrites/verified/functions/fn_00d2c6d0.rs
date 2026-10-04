// original: 0x00d2c6d0 CTaskComplexMoveReturnToRoute::vf20
/// Route poll for `ped`: when `[[ped+0x224]+0x264]` reaches the global
/// limit (signed), run the virtual kind on `this` and push it through the
/// three probe callees (the first two take singly-snapped frame slots that
/// hold zero), and return the sub-task at `+8`. Otherwise, when that
/// sub-task is set, of virtual kind 0x3ae, at stage >= 4 (`+0x99`), in
/// state 2 and its check on (`ped`, 1, 0) passes, forward (`0x384`, `ped`)
/// to the shared routine and return its answer; any failed gate returns
/// the sub-task.
///
/// Thiscall, one stack word (pointer).
lf_checker_rt::export!(thiscall, rw_00d2c6d0(this: u32, ped: u32) -> u32 {
    unsafe {
        const LIMIT_GLOB: u32 = 0x00e9d7fc;
        const WANT_KIND: u32 = 0x3ae;
        const WANT_STATE: u32 = 2;
        const MIN_STAGE: u8 = 4;
        const TAG: u32 = 0x384;
        const VT_KIND: u32 = 0x0c;
        const PROBE1: u32 = 1;
        const PROBE2: u32 = 2;
        const PROBE3: u32 = 3;
        const STATE: u32 = 4;
        const CHECK: u32 = 6;
        const ROUTINE: u32 = 7;
        let inner = ((ped + 0x224) as *const u32).read_unaligned();
        let v = ((inner + 0x264) as *const u32).read_unaligned();
        let lim = lf_checker_rt::global::<u32>(LIMIT_GLOB).read_unaligned();
        if (v as i32) >= (lim as i32) {
            let vt = (this as *const u32).read_unaligned();
            let kind_tgt = ((vt + VT_KIND) as *const u32).read_unaligned();
            let kind: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(kind_tgt as usize);
            let t = kind(this);
            let mut slot_a: u32 = 0;
            lf_checker_rt::callee_thiscall!(PROBE1, u32, &mut slot_a as *mut u32 as u32, t);
            let mut slot_b: u32 = 0;
            lf_checker_rt::callee_thiscall!(PROBE2, u32, inner.wrapping_add(0x84), &mut slot_b as *mut u32 as u32, 0, 1);
            let mut slot_c: u32 = 0;
            lf_checker_rt::callee_thiscall!(PROBE3, u32, &mut slot_c as *mut u32 as u32);
            ((this + 8) as *const u32).read_unaligned()
        } else {
            let sub = ((this + 8) as *const u32).read_unaligned();
            if sub == 0 {
                return 0;
            }
            let vt = (sub as *const u32).read_unaligned();
            let kind_tgt = ((vt + VT_KIND) as *const u32).read_unaligned();
            let kind: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(kind_tgt as usize);
            if kind(sub) != WANT_KIND {
                return sub;
            }
            if ((sub + 0x99) as *const u8).read() < MIN_STAGE {
                return sub;
            }
            let st: u32 = lf_checker_rt::callee_thiscall!(STATE, u32, sub);
            if st != WANT_STATE {
                return sub;
            }
            let ok: u32 = lf_checker_rt::callee_thiscall!(CHECK, u32, sub, ped, 1, 0);
            if ok & 0xff == 0 {
                return sub;
            }
            lf_checker_rt::callee_thiscall!(ROUTINE, u32, this, TAG, ped)
        }
    }
});
