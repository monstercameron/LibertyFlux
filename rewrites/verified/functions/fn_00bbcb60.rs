// original: 0x00BBCB60 ped_assign_drive_to_point_task (proposed)

/// Build a drive-to-point task for a ped and assign it at slot 0x0F.
///
/// `ped` resolves through the ped pool like the sibling builders; a zero
/// handle skips the lookup and a busy flag returns early with no task. A
/// positive (`> 0`, signed) vehicle handle resolves through the vehicle
/// pool; otherwise a null vehicle is used.
///
/// The height `z` is kept when above -100.0 or NaN (`jb` is taken on an
/// unordered comparison), else it comes from the ground
/// helper (callee 3) over `(x, y, 4)`, which returns its float on the x87
/// stack. A model lookup (callee 4) runs when `a24` is nonzero, taking a
/// frame pointer to a staged `-1` word and answering through it; the
/// answered word is read back like the original. A drive task (callee 6) is
/// then built on a fresh pool object as
/// `(vehicle, xyz_ptr, a1c_trunc, a20, looked_up, a2c, a28, edi, a34)` where
/// `a1c_trunc` is `a1c` truncated toward zero (exact `cvttss2si`
/// emulation), `xyz_ptr` stages `(x, y, z)`, and `edi` is `a30` unless
/// negative (signed), in which case `0x14`. The task is assigned as
/// `(ped, task, slot)` with slot `TASK_SLOT` (callee 7); a failed
/// allocation assigns null and skips the drive call.
///
/// Original: 0x00BBCB60 (cdecl, twelve stack words, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00BBCB60(ped: u32, veh: u32, x: u32, y: u32, z: u32, a1c: u32, a20: u32, a24: u32, a28: u32, a2c: u32, a30: u32, a34: u32) -> u32 {
    unsafe {
        const PED_POOL: u32 = 0x018B6F1C;
        const VEH_POOL: u32 = 0x012E22A4;
        const TASK_POOL: u32 = 0x0167E2A0;
        const MIN_Z: u32 = 0x00FE8DF8;
        const PED_STATUS: u32 = 0x6C;
        const PED_BUSY_FLAG: u32 = 0x0E;
        const TASK_SLOT: u32 = 0x0F;
        const EDI_DEFAULT: u32 = 0x14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn cvtt_truncate(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x80000000u32 as i32
            } else {
                x as i32
            }
        }

        if ped != 0 {
            let pool = rd32(lf_checker_rt::relocated(PED_POOL));
            let obj: u32 = lf_checker_rt::callee_thiscall!(1, u32, pool, ped);
            let status = rd32(obj.wrapping_add(PED_STATUS));
            if status != 0 && rd8(status.wrapping_add(PED_BUSY_FLAG)) != 0 {
                return 0;
            }
        }
        let vehicle: u32;
        if (veh as i32) > 0 {
            let vpool = rd32(lf_checker_rt::relocated(VEH_POOL));
            vehicle = lf_checker_rt::callee_thiscall!(2, u32, vpool, veh);
        } else {
            vehicle = 0;
        }
        let min_z = f32::from_bits(rd32(lf_checker_rt::relocated(MIN_Z)));
        let zf = f32::from_bits(z);
        let height: u32;
        if !(min_z >= zf) {
            height = z;
        } else {
            let g: f32 = lf_checker_rt::callee_cdecl!(3, f32, x, y, 4);
            height = g.to_bits();
        }
        let xyz = [x, y, height];
        let xyz_ptr = xyz.as_ptr() as u32;
        let mut lookup = [0xFFFFFFFFu32, vehicle];
        if a24 != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(4, u32, a24, lookup.as_mut_ptr() as u32);
        }
        let looked_up = lookup[0];
        let edi = if (a30 as i32) < 0 { EDI_DEFAULT } else { a30 };
        let task_pool = rd32(lf_checker_rt::relocated(TASK_POOL));
        let alloc: u32 = lf_checker_rt::callee_thiscall!(5, u32, task_pool);
        if alloc == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, ped, 0, TASK_SLOT);
        } else {
            let count = cvtt_truncate(f32::from_bits(a1c));
            let task: u32 = lf_checker_rt::callee_thiscall!(6, u32, alloc, vehicle, xyz_ptr, count as u32, a20, looked_up, a2c, a28, edi, a34);
            let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, ped, task, TASK_SLOT);
        }
        0
    }
});
