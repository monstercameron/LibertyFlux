// original: 0x00B729E0 CTaskSimpleCarShuffle::vf17

/// Car-shuffle task step: validate the ped is still in the task's car, pick a
/// shuffle target, and either clear or set the sub-task's state flags.
///
/// `this` is the task object: byte `DONE` at `+0x14` (nonzero means finished),
/// `SUB` at `+0x18` (pointer to a sub-task whose flags word at `+0x04` this
/// step reads and writes, may be null) and `CAR_HANDLE` at `+0x1c` (the car
/// the task acts on). `ped` is the ped object: `PED_CAR` at `+0xb30` (its
/// current car) and `PED_AUX` at `+0x6c` (pointer to an auxiliary object whose
/// byte at `+0x0e` gates one path, may be null).
///
/// Steps: return 1 at once when the task is done, has no car, or the ped's
/// car differs from the task's car. Otherwise ask callee 1
/// (thiscall on the car, ped argument) for a seat code and map it to a
/// shuffle index (0->2, 2->0, 1->3, 3->1, anything else -1), then ask callee
/// 2 (thiscall on the car, index argument) for the shuffle-target object,
/// which may be null. When there is no sub-task, report the attempt through
/// callee 3 (cdecl: ped, car handle, target, -1); when a target exists, check
/// it with callee 4 (thiscall, no stack arguments) and combine its low byte
/// with the target's `+0x219` byte into a status word whose upper 24 bits are
/// the check's full return value (or callee 3's when no target exists), then
/// report through callee 5 (thiscall on the task: ped, status word).
/// Callee 6 (cdecl, no arguments) then decides the outcome: when its low byte
/// is set and the auxiliary object is missing or its flag byte is clear, take
/// the set-flags path below; when the auxiliary object and its flag are
/// present, return 0, clearing the sub-task's `0x8000` flag first if set
/// (a null sub-task faults here, as in the original). When callee 6's low
/// byte is clear and a target exists, poll callee 7 (thiscall on
/// target`+0x224` plus `0x2e0`, arguments `0x2e2` and 0) and treat a nonzero
/// low byte like the missing-auxiliary case. Finally, when a target existed
/// and the outcome flag is set, clear the sub-task's `0x8000` flag if set and
/// notify through callees 8 (thiscall on the sub-task: the task) and 9
/// (thiscall on the sub-task: 2, the `0x00B6FE30` callback address the
/// original pushes, the task); otherwise set the sub-task's `0x8000` and
/// `0x10` flag bits and notify through the same two callees with a 1 instead
/// of a 2. Both paths return 0.
///
/// The original keeps the target-exists flag in the low byte of its own dead
/// incoming-argument slot; the rewrite uses a local for the value, and the
/// contract compares everything except that slot (see `narrowed`).
/// Original: 0x00B729E0 (thiscall, one stack word), returns an 8-bit boolean.
lf_checker_rt::export!(thiscall, rw_00B729E0(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_DONE: u32 = 0x14;
        const TASK_SUB: u32 = 0x18;
        const TASK_CAR: u32 = 0x1c;
        const PED_AUX: u32 = 0x6c;
        const AUX_FLAG: u32 = 0x0e;
        const PED_CAR: u32 = 0xb30;
        const TARGET_MARK: u32 = 0x219;
        const TARGET_BASE: u32 = 0x224;
        const TARGET_BIAS: u32 = 0x2e0;
        const SUB_FLAGS: u32 = 0x04;
        const FLAG_BUSY: u32 = 0x8000;
        const FLAG_EXTRA: u32 = 0x10;
        const POLL_ARG0: u32 = 0x2e2;
        const NOTIFY_CALLBACK: u32 = 0x00b6fe30;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if rd8(this.wrapping_add(TASK_DONE)) != 0 {
            return 1;
        }
        let car_handle = rd32(this.wrapping_add(TASK_CAR));
        if car_handle == 0 {
            return 1;
        }
        let car = rd32(ped.wrapping_add(PED_CAR));
        if car != car_handle {
            return 1;
        }
        let seat: u32 = lf_checker_rt::callee_thiscall!(1, u32, car, ped);
        let index: u32 = match seat {
            0 => 2,
            2 => 0,
            1 => 3,
            3 => 1,
            _ => 0xffff_ffff,
        };
        let target: u32 = lf_checker_rt::callee_thiscall!(2, u32, car, index);
        let target_exists: u32 = u32::from(target != 0);
        let sub = rd32(this.wrapping_add(TASK_SUB));
        if sub == 0 {
            let report: u32 =
                lf_checker_rt::callee_cdecl!(3, u32, ped, car_handle, target, 0xffff_ffff);
            let status: u32 = if target_exists != 0 {
                let check: u32 = lf_checker_rt::callee_thiscall!(4, u32, target);
                let flag: u32 = if check & 0xff != 0 {
                    1
                } else if rd8(target.wrapping_add(TARGET_MARK)) == 0 {
                    0
                } else {
                    1
                };
                (check & 0xffff_ff00) | flag
            } else {
                report & 0xffff_ff00
            };
            lf_checker_rt::callee_thiscall!(5, u32, this, ped, status);
        }
        let proceed: u32 = lf_checker_rt::callee_cdecl!(6, u32,);
        let mut outcome: u32 = 0;
        if proceed & 0xff != 0 {
            let aux = rd32(ped.wrapping_add(PED_AUX));
            if aux == 0 || rd8(aux.wrapping_add(AUX_FLAG)) == 0 {
                outcome = 1;
            } else {
                let sub_task = rd32(this.wrapping_add(TASK_SUB));
                let flags = rd32(sub_task.wrapping_add(SUB_FLAGS));
                if (flags >> 15) & 1 == 0 {
                    return 0;
                }
                wr32(sub_task.wrapping_add(SUB_FLAGS), flags & !FLAG_BUSY);
                return 0;
            }
        } else if target != 0 {
            let polled: u32 = lf_checker_rt::callee_thiscall!(
                7,
                u32,
                rd32(target.wrapping_add(TARGET_BASE)).wrapping_add(TARGET_BIAS),
                POLL_ARG0,
                0
            );
            if polled & 0xff != 0 {
                outcome = 1;
            }
        }
        if target_exists != 0 && outcome != 0 {
            let sub_task = rd32(this.wrapping_add(TASK_SUB));
            let flags = rd32(sub_task.wrapping_add(SUB_FLAGS));
            if (flags >> 15) & 1 != 0 {
                wr32(sub_task.wrapping_add(SUB_FLAGS), flags & !FLAG_BUSY);
            }
            let notify_on = rd32(this.wrapping_add(TASK_SUB));
            lf_checker_rt::callee_thiscall!(8, u32, notify_on, this);
            let notify_on = rd32(this.wrapping_add(TASK_SUB));
            lf_checker_rt::callee_thiscall!(9, u32, notify_on, 2, NOTIFY_CALLBACK, this);
            0
        } else {
            let sub_task = rd32(this.wrapping_add(TASK_SUB));
            wr32(
                sub_task.wrapping_add(SUB_FLAGS),
                rd32(sub_task.wrapping_add(SUB_FLAGS)) | FLAG_BUSY,
            );
            let sub_task = rd32(this.wrapping_add(TASK_SUB));
            wr32(
                sub_task.wrapping_add(SUB_FLAGS),
                rd32(sub_task.wrapping_add(SUB_FLAGS)) | FLAG_EXTRA,
            );
            let notify_on = rd32(this.wrapping_add(TASK_SUB));
            lf_checker_rt::callee_thiscall!(8, u32, notify_on, this);
            let notify_on = rd32(this.wrapping_add(TASK_SUB));
            lf_checker_rt::callee_thiscall!(9, u32, notify_on, 1, NOTIFY_CALLBACK, this);
            0
        }
    }
});
