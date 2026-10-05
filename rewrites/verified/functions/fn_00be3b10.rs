// original: 0x00be3b10 CTaskComplexControlMovement::vf18 (merged name)

/// Dispatch a movement-control task tick: forward, run, or deactivate.
///
/// `task` points to the task (`+0x28` flags, `+0x1c` stage, `+0x14` sub-task,
/// `+0x18`/`+0x8` watched objects, `+0x4` auxiliary object, `+0xc` mode bits,
/// vtable at `+0x0`); `ped` is the ped the task runs on.
///
/// When flag bit 3 is set it is cleared and the tick is forwarded to the
/// base handler (callee 1, thiscall with the ped), whose answer is returned.
/// Otherwise the task deactivates when flag bit 4 or bit 2 is set, the stage
/// is 1 or 2, or there is no sub-task; deactivation first bails out with 0
/// when mode bit 1 is set, then asks the ped's manager (callee 2, thiscall
/// on `[ped + 0x224] + 0x44` with argument 1) and bails out with 0 on a null
/// answer. With an auxiliary object whose type slot (`+0xc`) reads 0x3c5 the
/// manager is asked again and the answer is driven through virtual slots
/// `+0x30` and `+0x10`; otherwise the manager is re-armed (callee 3, four
/// stack arguments), the arming is acknowledged (callee 4), and bit 0 of the
/// manager's status word (`+0xc` of a fresh answer) is set. Every
/// deactivation path returns 0.
///
/// When the task stays active and the watched object at `+0x18` is non-null,
/// the type slots of `+0x8` and `+0x18` are compared; on a match the `+0x18`
/// object is released through virtual slot `+0` with argument 1 and the slot
/// is cleared. The tick then runs through the task's own virtual slot
/// `+0x54` with the ped, whose answer is returned.
///
/// All virtual calls go through the same fabricated objects on both sides,
/// landing on the checker's planted stubs.
///
/// Original: 0x00be3b10 (thiscall, ecx = task, one stack word = ped).
lf_checker_rt::export!(thiscall, rw_00be3b10(task: u32, ped: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x28;
        const STAGE: u32 = 0x1c;
        const SUBTASK: u32 = 0x14;
        const WATCHED: u32 = 0x18;
        const OTHER: u32 = 0x8;
        const AUX: u32 = 0x4;
        const MODE: u32 = 0xc;
        const PED_MGR: u32 = 0x224;
        const MGR_INNER: u32 = 0x44;
        const MGR_STATUS: u32 = 0xc;
        const AUX_WANTED: u32 = 0x3c5;
        const SLOT_TYPE: u32 = 0xc;
        const SLOT_RELEASE: u32 = 0x0;
        const SLOT_RUN: u32 = 0x54;
        const SLOT_OPEN: u32 = 0x30;
        const SLOT_USE: u32 = 0x10;
        const BASE_CALLEE: u32 = 1;
        const MGR_CALLEE: u32 = 2;
        const REARM_CALLEE: u32 = 3;
        const ACK_CALLEE: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// Virtual call with no stack arguments through the object's table.
        #[inline(always)]
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                f(obj)
            }
        }
        /// Virtual call with one stack argument through the object's table.
        #[inline(always)]
        unsafe fn vcall1(obj: u32, slot: u32, a: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                f(obj, a)
            }
        }

        let flags = rd32(task.wrapping_add(FLAGS));
        if flags & 8 != 0 {
            wr32(task.wrapping_add(FLAGS), flags & !8);
            return lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, task, ped);
        }
        let stage = rd32(task.wrapping_add(STAGE));
        if flags & 0x10 != 0
            || stage == 1
            || stage == 2
            || rd32(task.wrapping_add(SUBTASK)) == 0
            || flags & 4 != 0
        {
            if rd32(task.wrapping_add(MODE)) & 2 != 0 {
                return 0;
            }
            let mgr = rd32(ped.wrapping_add(PED_MGR)).wrapping_add(MGR_INNER);
            let first: u32 = lf_checker_rt::callee_thiscall!(MGR_CALLEE, u32, mgr, 1u32);
            if first == 0 {
                return 0;
            }
            let aux = rd32(task.wrapping_add(AUX));
            if aux != 0 && vcall0(aux, SLOT_TYPE) == AUX_WANTED {
                let opened: u32 = lf_checker_rt::callee_thiscall!(MGR_CALLEE, u32, mgr, 1u32);
                let handle = vcall0(opened, SLOT_OPEN);
                vcall1(handle, SLOT_USE, 1);
                return 0;
            }
            let rearmed: u32 =
                lf_checker_rt::callee_thiscall!(REARM_CALLEE, u32, mgr, 1u32, ped, 2u32, 0u32);
            let _: u32 = lf_checker_rt::callee_thiscall!(ACK_CALLEE, u32, rearmed);
            let status: u32 = lf_checker_rt::callee_thiscall!(MGR_CALLEE, u32, mgr, 1u32);
            wr32(
                status.wrapping_add(MGR_STATUS),
                rd32(status.wrapping_add(MGR_STATUS)) | 1,
            );
            return 0;
        }
        if rd32(task.wrapping_add(WATCHED)) != 0 {
            let a = vcall0(rd32(task.wrapping_add(OTHER)), SLOT_TYPE);
            let b = vcall0(rd32(task.wrapping_add(WATCHED)), SLOT_TYPE);
            if a == b {
                let watched = rd32(task.wrapping_add(WATCHED));
                if watched != 0 {
                    vcall1(watched, SLOT_RELEASE, 1);
                }
                wr32(task.wrapping_add(WATCHED), 0);
            }
        }
        vcall1(task, SLOT_RUN, ped)
    }
});
