// original: 0x00cb36f0 CTaskComplexMoveBeInFormation::vf18

/// Pick the stay-in-formation subtask from the child type and alignment.
///
/// `this` is the complex task, `ped` the pedestrian. The child type comes
/// from virtual slot 3 of the sub-object at `+0x08` (queried twice); the
/// pedestrian carries a stance byte at `+0x1e2`, a heading float at
/// `+0xaa4`, and a group pointer at `+0xab0`.
///
/// Dispatch on the child type: `0x386` and `0x3ae` request `0x386`;
/// `0x38b` requests `0x11a`; `0x11a`, `0x388` and `0x3b7` continue below;
/// anything else returns 0. The second query, when `0x11a` with the stance
/// nibble at 2 or more, requests `0x11a`. Otherwise a group flag is formed
/// (group present, `+0x28` masked by `0x3c0` equal to `0x80`, `+0x1304`
/// equal to 2): with the check callee (id 3) agreeing and the flag clear,
/// the target callee (id 4) feeds the measure callee (id 5), whose nonzero
/// (NaN counts as nonzero) answer requests `0x3ae` and zero requests
/// `0x3b7`. In every other case the two angle callees (ids 6, 7) run on
/// (`+0x60`, heading) and the absolute value is compared against
/// `0x3d8efa35`: not-above requests `0x11a`, above requests `0x386`.
///
/// The angle path spills float temporaries into the dead argument slot, so
/// the contract disables the stack check (recorded in `narrowed`); the
/// spilled values are already observed through the call logs. The
/// not-above edge is written as `!(a > b)` so NaN takes the original's
/// branch.
///
/// Original: 0x00cb36f0 (thiscall, one stack word; returns the subtask id).
lf_checker_rt::export!(thiscall, rw_00cb36f0(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 0x08;
        const VTABLE_SLOT: u32 = 0x0c;
        const ALIGN: u32 = 0x60;
        const STANCE: u32 = 0x1e2;
        const HEADING: u32 = 0xaa4;
        const GROUP: u32 = 0xab0;
        const GROUP_FLAGS: u32 = 0x28;
        const GROUP_FLAGS_MASK: u32 = 0x3c0;
        const GROUP_FLAGS_WANT: u32 = 0x80;
        const GROUP_READY: u32 = 0x1304;
        const GROUP_READY_WANT: u32 = 2;
        const ANGLE_LIMIT: f32 = f32::from_bits(0x3d8efa35);
        const FORM_TASK: u32 = 0x11a;
        const CHASE_TASK: u32 = 0x386;
        const AIM_TASK: u32 = 0x3ae;
        const HOLD_TASK: u32 = 0x3b7;
        const CHECK_CALLEE: u32 = 3;
        const TARGET_CALLEE: u32 = 4;
        const MEASURE_CALLEE: u32 = 5;
        const ANGLE_A_CALLEE: u32 = 6;
        const ANGLE_B_CALLEE: u32 = 7;
        const REQUEST_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        let sub = rd32(this + SUBTASK);
        let slot: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(sub) + VTABLE_SLOT) as usize);
        let child = slot(sub);
        if child > 0x38b {
            if child == 0x3ae {
                return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, CHASE_TASK, ped);
            }
            if child != HOLD_TASK {
                return 0;
            }
        } else if child == 0x38b {
            return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, FORM_TASK, ped);
        } else {
            if child == FORM_TASK || child == 0x388 {
            } else if child == CHASE_TASK {
                return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, CHASE_TASK, ped);
            } else {
                return 0;
            }
        }
        if slot(sub) == FORM_TASK {
            let stance = ((ped + STANCE) as *const u8).read() & 0x0f;
            if stance >= 2 {
                return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, FORM_TASK, ped);
            }
        }
        let grp = rd32(ped + GROUP);
        let flag: u8 = if grp != 0
            && rd32(grp + GROUP_FLAGS) & GROUP_FLAGS_MASK == GROUP_FLAGS_WANT
            && rd32(grp + GROUP_READY) == GROUP_READY_WANT
        {
            1
        } else {
            0
        };
        let ok = (lf_checker_rt::callee_thiscall!(CHECK_CALLEE, u32, this, ped) & 0xff) as u8;
        if ok != 0 && flag == 0 {
            let tgt: u32 = lf_checker_rt::callee_thiscall!(TARGET_CALLEE, u32, this);
            let m: f32 = lf_checker_rt::callee_thiscall!(MEASURE_CALLEE, f32, tgt);
            if m != 0.0 {
                return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, AIM_TASK, ped);
            }
            return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, HOLD_TASK, ped);
        }
        let g: f32 = lf_checker_rt::callee_cdecl!(
            ANGLE_A_CALLEE,
            f32,
            rdf(this + ALIGN).to_bits(),
            rdf(ped + HEADING).to_bits()
        );
        let h: f32 =
            lf_checker_rt::callee_cdecl!(ANGLE_B_CALLEE, f32, g.to_bits());
        if !(h.abs() > ANGLE_LIMIT) {
            return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, FORM_TASK, ped);
        }
        lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, CHASE_TASK, ped)
    }
});
