// original: 0x00BBDCA0 ped_assign_gang_driveby_task (proposed)

/// Build a gang-driveby task for a ped and assign it at slot 0x34.
///
/// `ped` resolves through the ped pool like the sibling builders; a zero
/// handle skips the lookup and a busy flag returns early with no task. Two
/// optional handles resolve the same way: `ac` (ped pool) fills two frame
/// slots, `a10` (vehicle pool) overwrites the first. (A redundant
/// read-write-back of an unstaged frame word between the two stagings has
/// no observable effect and is not reproduced.)
///
/// When `ped` is set, it is resolved again and followed through `+0x224`
/// and `+0x50`; a live object there is asked for its task id through the
/// virtual slot at `+0x0C` (callee 5, reached through a planted stub on
/// both sides). An answer of `0x419` whose object word at `+0x20` equals
/// the `ac` slot returns early: the ped already runs this task.
///
/// Otherwise a driveby task (callee 7) is built on a fresh pool object as
/// `([F+8], xyz_ptr, a20, a2c, a24, a28)` over the staged
/// `(a14, a18, a1c)` floats, and assigned as `(ped, task, slot)` with slot
/// `TASK_SLOT` (callee 8); a failed allocation assigns null.
///
/// Original: 0x00BBDCA0 (cdecl, ten stack words, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00BBDCA0(ped: u32, ac: u32, a10: u32, a14: u32, a18: u32, a1c: u32, a20: u32, a24: u32, a28: u32, a2c: u32) -> u32 {
    unsafe {
        const PED_POOL: u32 = 0x018B6F1C;
        const VEH_POOL: u32 = 0x012E22A4;
        const TASK_POOL: u32 = 0x0167E2A0;
        const PED_STATUS: u32 = 0x6C;
        const PED_BUSY_FLAG: u32 = 0x0E;
        const INTEL_OFF: u32 = 0x224;
        const TASK_IFACE_OFF: u32 = 0x50;
        const VTASK_ID_SLOT: u32 = 0x0C;
        const TASK_MATCH_WORD: u32 = 0x20;
        const DRIVEBY_TASK_ID: u32 = 0x419;
        const TASK_SLOT: u32 = 0x34;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let ped_pool = rd32(lf_checker_rt::relocated(PED_POOL));
        if ped != 0 {
            let obj: u32 = lf_checker_rt::callee_thiscall!(1, u32, ped_pool, ped);
            let status = rd32(obj.wrapping_add(PED_STATUS));
            if status != 0 && rd8(status.wrapping_add(PED_BUSY_FLAG)) != 0 {
                return 0;
            }
        }
        let mut f8 = 0u32;
        let mut fc = 0u32;
        if ac != 0 {
            let ans: u32 = lf_checker_rt::callee_thiscall!(2, u32, ped_pool, ac);
            fc = ans;
            f8 = ans;
        }
        if a10 != 0 {
            let vpool = rd32(lf_checker_rt::relocated(VEH_POOL));
            f8 = lf_checker_rt::callee_thiscall!(3, u32, vpool, a10);
        }
        if ped != 0 {
            let obj: u32 = lf_checker_rt::callee_thiscall!(4, u32, ped_pool, ped);
            let intel = rd32(obj.wrapping_add(INTEL_OFF));
            let iface = rd32(intel.wrapping_add(TASK_IFACE_OFF));
            if iface != 0 {
                let vtable = rd32(iface);
                let slot = rd32(vtable.wrapping_add(VTASK_ID_SLOT));
                let f: extern "thiscall" fn(u32) -> u32 =
                    unsafe { core::mem::transmute(slot as usize) };
                let tid = f(iface);
                if tid == DRIVEBY_TASK_ID && rd32(iface.wrapping_add(TASK_MATCH_WORD)) == fc {
                    return 0;
                }
            }
        }
        let task_pool = rd32(lf_checker_rt::relocated(TASK_POOL));
        let xyz = [a14, a18, a1c];
        let xyz_ptr = xyz.as_ptr() as u32;
        let alloc: u32 = lf_checker_rt::callee_thiscall!(6, u32, task_pool);
        if alloc == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(8, u32, ped, 0, TASK_SLOT);
        } else {
            let task: u32 = lf_checker_rt::callee_thiscall!(7, u32, alloc, f8, xyz_ptr, a20, a2c, a24, a28);
            let _: u32 = lf_checker_rt::callee_cdecl!(8, u32, ped, task, TASK_SLOT);
        }
        0
    }
});
