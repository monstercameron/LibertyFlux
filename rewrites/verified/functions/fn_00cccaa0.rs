// original: 0x00cccaa0 CTaskComplexRevive::vf18

/// Revive-task tick: dispatch on the sub-task's kind id.
///
/// `task` points to the revive task (`+0x8` sub-task pointer, `+0x18` state
/// word); `ped` is the ped the task runs on (`+0x7b8` state word, `+0x570`
/// pose block, vtable at `+0x0`). Original is thiscall: ecx = task, one
/// stack word = ped, callee pops 4.
///
/// The tick reads the kind id through the sub-task's virtual slot `+0xc`
/// (callee 1) and dispatches with a SIGNED greater-than against 734
/// (`cmp`/`jg`; equality below is signedness-free):
/// - kind 206 or 2117: drive the ped's pose block through the pose helper
///   (callee 7, thiscall on `ped + 0x570`, ten stack words: a pose-table
///   address, 1, 0, 0, -1, 0, 0, 1.0f, 0, 0), then finish only when the
///   task state word is nonzero, else return 0.
/// - kind 281: when the ped state word reads 6, take the manager from the
///   globals word (callee 2 answers it, thiscall with no stack words) and
///   return the revive-start call on it (callee 3, args 2 and 0x67);
///   otherwise run the ped's virtual slots `+0xb0` then `+0xac` (callees 5
///   and 6), take the manager the same way and return the revive-step call
///   on it (callee 4, no stack words). A null manager returns 0.
/// - any other kind (including 734 itself): return 0.
///
/// Finishing (callee 8, thiscall on the manager): five stack words, the task
/// state word, -3, 0x1b, 8, 0; its answer is returned.
///
/// Signedness note: the `> 734` test is signed (`jg`). An unsigned test
/// would differ only for kind ids above 0x7fffffff, and every one of those
/// returns 0 either way (the sub-chain matches none of 206/281 and the
/// greater-branch matches none of 2117), so the two comparisons are
/// observably identical on all inputs; the contract still feeds negative
/// and extreme ids to pin the dispatch shape.
///
/// All virtual calls go through the same fabricated objects on both sides,
/// landing on the checker's planted stubs.
lf_checker_rt::export!(thiscall, rw_00cccaa0(task: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 0x8;
        const TASK_STATE: u32 = 0x18;
        const PED_STATE: u32 = 0x7b8;
        const PED_POSE: u32 = 0x570;
        const SLOT_KIND: u32 = 0xc;
        const SLOT_PRE_A: u32 = 0xb0;
        const SLOT_PRE_B: u32 = 0xac;
        const KIND_POSE_A: i32 = 206;
        const KIND_REVIVE: i32 = 281;
        const KIND_DEAD: i32 = 734;
        const KIND_POSE_B: i32 = 2117;
        const PED_READY: u32 = 6;
        const POSE_TABLE_A: u32 = 0x00ed9c08;
        const POSE_TABLE_B: u32 = 0x00ed9c10;
        const ONE_BITS: u32 = 0x3f800000;
        const MANAGER_GLOBAL: u32 = 0x0167e2a0;
        const MANAGER_CALLEE: u32 = 2;
        const START_CALLEE: u32 = 3;
        const STEP_CALLEE: u32 = 4;
        const POSE_CALLEE: u32 = 7;
        const FINISH_CALLEE: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        #[inline(always)]
        unsafe fn manager() -> u32 {
            unsafe {
                let g = lf_checker_rt::global::<u32>(MANAGER_GLOBAL);
                lf_checker_rt::callee_thiscall!(
                    MANAGER_CALLEE,
                    u32,
                    (g as *const u32).read_unaligned()
                )
            }
        }
        #[inline(always)]
        unsafe fn drive_pose(ped: u32, table: u32) {
            unsafe {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    POSE_CALLEE,
                    u32,
                    ped.wrapping_add(PED_POSE),
                    lf_checker_rt::relocated(table),
                    1u32,
                    0u32,
                    0u32,
                    0xffffffffu32,
                    0u32,
                    0u32,
                    ONE_BITS,
                    0u32,
                    0u32
                );
            }
        }
        #[inline(always)]
        unsafe fn finish(task: u32, mgr: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_thiscall!(
                    FINISH_CALLEE,
                    u32,
                    mgr,
                    rd32(task.wrapping_add(TASK_STATE)),
                    0xfffffffdu32,
                    0x1bu32,
                    8u32,
                    0u32
                )
            }
        }

        let sub = rd32(task.wrapping_add(SUBTASK));
        let kind: u32 = vcall0(sub, SLOT_KIND);
        let k = kind as i32;
        if k > KIND_DEAD {
            if k != KIND_POSE_B {
                return 0;
            }
            drive_pose(ped, POSE_TABLE_B);
            if rd32(task.wrapping_add(TASK_STATE)) == 0 {
                return 0;
            }
            let mgr = manager();
            if mgr == 0 {
                return 0;
            }
            return finish(task, mgr);
        }
        if k == KIND_DEAD {
            return 0;
        }
        // Equality below: the original subtracts and jumps on zero, so the
        // comparison is the same signed or unsigned; compare raw words.
        if kind == KIND_POSE_A as u32 {
            drive_pose(ped, POSE_TABLE_A);
            if rd32(task.wrapping_add(TASK_STATE)) == 0 {
                return 0;
            }
            let mgr = manager();
            if mgr == 0 {
                return 0;
            }
            return finish(task, mgr);
        }
        if kind != KIND_REVIVE as u32 {
            return 0;
        }
        if rd32(ped.wrapping_add(PED_STATE)) == PED_READY {
            let mgr = manager();
            if mgr == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(START_CALLEE, u32, mgr, 2u32, 0x67u32);
        }
        let _: u32 = vcall0(ped, SLOT_PRE_A);
        let _: u32 = vcall0(ped, SLOT_PRE_B);
        let mgr = manager();
        if mgr == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(STEP_CALLEE, u32, mgr)
    }
});
