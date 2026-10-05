// original: 0x00970000 ped_audio_task_gate (proposed)

/// Probe the local player's task list and latch two audio flags plus a smoothed level.
///
/// `this` is an audio-side object holding the outputs: a result flag at
/// `+0x3024`, an enable flag at `+0x3000`, a smoothed level at `+0x3004`
/// and the smoother state at `+0x3008`. The function asks the script
/// manager (callee 0) for its state block and the player ped (callee 1)
/// for its task owner (`+0x224`), then scans the owner's node list
/// (`+0x2e0`; each node holds an id at `+4`, flag bits at `+8`, the next
/// node at `+12`) twice: first for the primary task id `0x2de`, and only
/// when that scan finds nothing, for the secondary id `0x2e2`. Both
/// scans track the running minimum of the rank `(flags >> 1) & 7` and
/// stop early once that minimum is already 2 or more while below the
/// current node's rank.
///
/// A found task is resolved through the task finder (callee 2). The
/// enable flag clears when the primary task's state (`+0x34`) is below
/// `0x18`, or when no primary task matched and the secondary task's
/// state reaches `0xb`; a null find leaves it set. The result flag is 1
/// exactly when the manager is non-null, its state word (`+0x1304`) is 3
/// and the enable flag is set. The target level (1.0 when enabled, else
/// 0.0) and the sample clock global are fed to the smoother (callee 3),
/// whose float answer is stored at `+0x3004`. The function returns
/// without setting AL after that final call, so the observed AL is the
/// callee answer's low byte (the stored level's low byte); the computed
/// 0/1 result itself is the value stored at `+0x3024` (thiscall, no
/// stack arguments).
lf_checker_rt::export!(thiscall, rw_00970000(this: u32) -> u32 {
    unsafe {
        const PED_TASKS: u32 = 0x224;
        const OWNER_HEAD: u32 = 0x2e0;
        const FIND_BASE: u32 = 0x44;
        const NODE_ID: u32 = 0x04;
        const NODE_FLAGS: u32 = 0x08;
        const NODE_NEXT: u32 = 0x0c;
        const PRIMARY_TASK: u32 = 0x2de;
        const SECONDARY_TASK: u32 = 0x2e2;
        const PRIMARY_LIMIT: u32 = 0x18;
        const SECONDARY_LIMIT: u32 = 0x0b;
        const TASK_STATE: u32 = 0x34;
        const MGR_STATE: u32 = 0x1304;
        const MGR_READY: u32 = 3;
        const FLAG_RESULT: u32 = 0x3024;
        const FLAG_ENABLE: u32 = 0x3000;
        const LEVEL_VALUE: u32 = 0x3004;
        const LEVEL_SMOOTHER: u32 = 0x3008;
        const SAMPLE_CLOCK: u32 = 0x11618fc;
        const CALLEE_MGR: u32 = 0;
        const CALLEE_PED: u32 = 1;
        const CALLEE_FIND: u32 = 2;
        const CALLEE_SMOOTH: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        /// Rank bits used by both scans: bits 3..1 of the flag word.
        #[inline(always)]
        unsafe fn rank(node: u32) -> u32 {
            unsafe { (rd32(node + NODE_FLAGS) >> 1) & 7 }
        }

        /// Scan the list starting at `head` for `wanted`, stopping early
        /// when the running minimum rank is below a node's rank while
        /// already at least 2. True when a node with the id is found.
        unsafe fn scan(head: u32, wanted: u32) -> bool {
            unsafe {
                let mut min = rank(head);
                let mut node = head;
                loop {
                    let v = rank(node);
                    if min >= v || min < 2 {
                        if rd32(node + NODE_ID) == wanted {
                            return true;
                        }
                        let next = rd32(node + NODE_NEXT);
                        if next == 0 {
                            return false;
                        }
                        node = next;
                        min = v;
                    } else {
                        return false;
                    }
                }
            }
        }

        let mgr: u32 = lf_checker_rt::callee_cdecl!(CALLEE_MGR, u32, 0);
        let mut enabled: u8 = 1;
        let result: u8;
        if mgr == 0 {
            enabled = 0;
            result = 0;
        } else {
            let ped: u32 = lf_checker_rt::callee_cdecl!(CALLEE_PED, u32,);
            let owner = rd32(ped + PED_TASKS);
            let head = rd32(owner + OWNER_HEAD);
            if head != 0 && scan(head, PRIMARY_TASK) {
                let task: u32 =
                    lf_checker_rt::callee_thiscall!(CALLEE_FIND, u32, owner + FIND_BASE, PRIMARY_TASK);
                if task != 0 && rd32(task + TASK_STATE) < PRIMARY_LIMIT {
                    enabled = 0;
                }
            } else if head != 0 && scan(head, SECONDARY_TASK) {
                let task: u32 =
                    lf_checker_rt::callee_thiscall!(CALLEE_FIND, u32, owner + FIND_BASE, SECONDARY_TASK);
                if task != 0 && rd32(task + TASK_STATE) >= SECONDARY_LIMIT {
                    enabled = 0;
                }
            }
            if rd32(mgr + MGR_STATE) == MGR_READY && enabled != 0 {
                result = 1;
            } else {
                result = 0;
            }
        }
        ((this + FLAG_RESULT) as *mut u8).write(result);
        ((this + FLAG_ENABLE) as *mut u8).write(enabled);
        let target: f32 = if enabled != 0 { 1.0 } else { 0.0 };
        let clock = (lf_checker_rt::global::<u32>(SAMPLE_CLOCK) as *const u32).read_unaligned();
        let level: f32 = lf_checker_rt::callee_thiscall!(
            CALLEE_SMOOTH,
            f32,
            this + LEVEL_SMOOTHER,
            target.to_bits(),
            clock
        );
        ((this + LEVEL_VALUE) as *mut f32).write_unaligned(level);
        // AL is whatever the smoother call left in EAX: its answer's low byte.
        (level.to_bits() & 0xFF) as u32
    }
});
