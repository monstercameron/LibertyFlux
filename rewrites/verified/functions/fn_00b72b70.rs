// original: 0x00B72B70 CTaskSimpleCarSlowBeDraggedOut::vf17

/// Slow-dragged-out-of-car task step: when the task is flagged, rebuild the
/// exit task through a scratch object and drop the sub-task; otherwise either
/// report through the sub-task path or refresh a car timer and notify.
///
/// `this` is the task object: byte `DONE` at `+0x14`, `SUB` at `+0x18`
/// (pointer to a sub-task with a vtable at `+0x00`, a float at `+0x4c` and a
/// flags word at `+0x74`, may be null), words `A`/`B`/`C`/`D`/`E` at
/// `+0x1c`/`+0x20`/`+0x24`/`+0x28`/`+0x2c`. `ped` is the ped object: words at
/// `+0x6c` and `+0x78`, bytes at `+0x218`/`+0x219`, `CAR` at `+0x228`
/// (pointer to a car object with a timer word at `+0x568`) and a flags byte
/// at `+0x26c` (bit 2 gates the car paths). A game global word (file address
/// `0x011735B4`) seeds the timer refresh.
///
/// Steps: return 1 at once when `C` is null. When `DONE` is set and the
/// ped's car bit is set, construct a scratch exit-task object in a stack
/// buffer through callee 1 (thiscall: `C`, `D`, 1, 1), run it for the ped
/// through callee 2 (thiscall on the buffer: the ped) and tear it down
/// through callee 3 (thiscall on the buffer); then, when a sub-task exists,
/// release it through callee 4 (thiscall on the sub-task: the task) and clear
/// the slot, and return 1. When `DONE` is clear and no sub-task exists,
/// refresh the timer when the ped's `+0x218` byte is clear and its `+0x219`
/// byte is set (`car[0x568] = max(car[0x568], global + 0xea60)`, unsigned),
/// report through callee 5 (thiscall on the task: the ped) and return 0.
/// When a sub-task exists and the car bit is clear, skip to the tail. When
/// the car bit is set and the sub-task's `0x80000` flag is clear, call the
/// sub-task's vtable slot 2 with the `+0x4c` float (which arrives in the
/// vector register on the original side), take its single-float result `r`
/// and compute `r * 2.0 + f` in that order; unless the sum is at least 1.0
/// (an unordered result also fails this test), skip to the tail. Otherwise
/// ask callee 6 (cdecl: `B`, `A + 1`) to proceed, stopping at the tail on a
/// zero answer, then poll callee 7 (thiscall on the ped): on a zero low byte
/// rebuild the scratch exit-task as above, otherwise prod the `+0x6c` object
/// through callee 8 (thiscall: 0). Then notify through callee 9 (thiscall on
/// the sub-task: the constant `0xC47A0000`) and callee 4 again, build the
/// replacement through callee 10 (thiscall on the ped's `+0x78` object: `B`,
/// `A + 1`, `0x447A0000`, -1), store it in the sub-task slot and notify
/// through callee 11 (thiscall on it: 1, the callback at file address
/// `0x00B6FE50` the original pushes as a relocated immediate, the task).
/// The tail returns 1 when `C` is null (unreachable: the entry check and the
/// absence of any store to `C` keep it non-null), returns 0 when `E` is set,
/// and otherwise finishes through callee 12 (thiscall on `C`: the ped, `D`,
/// the sub-task, 0) and returns 0.
///
/// The scratch buffer lives below the incoming stack pointer on both sides,
/// so its address is excluded from the call comparison; its contents are
/// never observed (the callees that take it are scripted). The two float
/// constants are the file's read-only values, used as literals.
/// Original: 0x00B72B70 (thiscall, one stack word), returns an 8-bit boolean.
lf_checker_rt::export!(thiscall, rw_00B72B70(this: u32, ped: u32) -> u32 {
    /// Shared tail: finish or delegate to callee 12.
    unsafe fn tail(task: u32, ped: u32) -> u32 {
        unsafe {
            const TASK_SUB: u32 = 0x18;
            const TASK_D: u32 = 0x28;
            const TASK_E: u32 = 0x2c;
            const TASK_C: u32 = 0x24;
            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            let c = rd32(task.wrapping_add(TASK_C));
            if c == 0 {
                return 1;
            }
            if rd32(task.wrapping_add(TASK_E)) != 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(
                12,
                u32,
                c,
                ped,
                rd32(task.wrapping_add(TASK_D)),
                rd32(task.wrapping_add(TASK_SUB)),
                0
            );
            0
        }
    }
    unsafe {
        const TASK_DONE: u32 = 0x14;
        const TASK_SUB: u32 = 0x18;
        const TASK_A: u32 = 0x1c;
        const TASK_B: u32 = 0x20;
        const TASK_C: u32 = 0x24;
        const TASK_D: u32 = 0x28;
        const TASK_E: u32 = 0x2c;
        const SUB_VTABLE: u32 = 0x00;
        const SUB_VALUE: u32 = 0x4c;
        const SUB_FLAGS: u32 = 0x74;
        const VTABLE_SLOT: u32 = 0x08;
        const FLAG_SKIP_POLL: u32 = 0x80000;
        const PED_AUX: u32 = 0x6c;
        const PED_TARGET: u32 = 0x78;
        const PED_MARK0: u32 = 0x218;
        const PED_MARK1: u32 = 0x219;
        const PED_CAR: u32 = 0x228;
        const PED_CAR_FLAGS: u32 = 0x26c;
        const CAR_BIT: u8 = 0x04;
        const CAR_TIMER: u32 = 0x568;
        const TIMER_BIAS: u32 = 0xea60;
        const GLOBAL_SEED: u32 = 0x011735b4;
        const SCALE: f32 = 2.0;
        const LIMIT: f32 = 1.0;
        const NOTE_ARG: u32 = 0xc47a0000;
        const BUILD_ARG2: u32 = 0x447a0000;
        const NOTIFY_CALLBACK_FILE_VA: u32 = 0x00b6fe50;

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
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// Rebuild the scratch exit-task object for the ped (callees 1-3).
        #[inline(always)]
        unsafe fn rebuild(task: u32, ped: u32) {
            unsafe {
                let mut scratch = [0u32; 9];
                let buf = scratch.as_mut_ptr() as u32;
                let c = rd32(task.wrapping_add(TASK_C));
                let d = rd32(task.wrapping_add(TASK_D));
                lf_checker_rt::callee_thiscall!(1, u32, buf, c, d, 1, 1);
                lf_checker_rt::callee_thiscall!(2, u32, buf, ped);
                lf_checker_rt::callee_thiscall!(3, u32, buf);
            }
        }

        let notify_callback: u32 = lf_checker_rt::relocated(NOTIFY_CALLBACK_FILE_VA);
        if rd32(this.wrapping_add(TASK_C)) == 0 {
            return 1;
        }
        if rd8(this.wrapping_add(TASK_DONE)) != 0 {
            if rd8(ped.wrapping_add(PED_CAR_FLAGS)) & CAR_BIT != 0 {
                rebuild(this, ped);
            }
            if rd32(this.wrapping_add(TASK_SUB)) != 0 {
                lf_checker_rt::callee_thiscall!(
                    4,
                    u32,
                    rd32(this.wrapping_add(TASK_SUB)),
                    this
                );
                wr32(this.wrapping_add(TASK_SUB), 0);
            }
            return 1;
        }
        let sub = rd32(this.wrapping_add(TASK_SUB));
        if sub == 0 {
            if rd8(ped.wrapping_add(PED_MARK0)) == 0
                && rd8(ped.wrapping_add(PED_MARK1)) != 0
            {
                let car = rd32(ped.wrapping_add(PED_CAR));
                let seed: u32 =
                    (lf_checker_rt::global::<u32>(GLOBAL_SEED) as *const u32).read();
                let mut timer = seed.wrapping_add(TIMER_BIAS);
                let old = rd32(car.wrapping_add(CAR_TIMER));
                if old > timer {
                    timer = old;
                }
                wr32(car.wrapping_add(CAR_TIMER), timer);
            }
            lf_checker_rt::callee_thiscall!(5, u32, this, ped);
            return 0;
        }
        if rd8(ped.wrapping_add(PED_CAR_FLAGS)) & CAR_BIT != 0 {
            if rd32(sub.wrapping_add(SUB_FLAGS)) & FLAG_SKIP_POLL == 0 {
                let vtable = rd32(sub.wrapping_add(SUB_VTABLE));
                let slot = rd32(vtable.wrapping_add(VTABLE_SLOT));
                let sample = rdf(sub.wrapping_add(SUB_VALUE));
                let poll: extern "thiscall" fn(u32, u32) -> f32 =
                    core::mem::transmute(slot as usize);
                let got: f32 = poll(sub, sample.to_bits());
                let total = add(mul(got, SCALE), sample);
                if !(total >= LIMIT) {
                    return tail(this, ped);
                }
            }
            let a = rd32(this.wrapping_add(TASK_A));
            let b = rd32(this.wrapping_add(TASK_B));
            let go: u32 = lf_checker_rt::callee_cdecl!(6, u32, b, a.wrapping_add(1));
            if go == 0 {
                return tail(this, ped);
            }
            let check: u32 = lf_checker_rt::callee_thiscall!(7, u32, ped);
            if check & 0xff == 0 {
                rebuild(this, ped);
            } else {
                lf_checker_rt::callee_thiscall!(
                    8,
                    u32,
                    rd32(ped.wrapping_add(PED_AUX)),
                    0
                );
            }
            lf_checker_rt::callee_thiscall!(
                9,
                u32,
                rd32(this.wrapping_add(TASK_SUB)),
                NOTE_ARG
            );
            lf_checker_rt::callee_thiscall!(
                4,
                u32,
                rd32(this.wrapping_add(TASK_SUB)),
                this
            );
            let a = rd32(this.wrapping_add(TASK_A));
            let b = rd32(this.wrapping_add(TASK_B));
            let built: u32 = lf_checker_rt::callee_thiscall!(
                10,
                u32,
                rd32(ped.wrapping_add(PED_TARGET)),
                b,
                a.wrapping_add(1),
                BUILD_ARG2,
                0xffff_ffff
            );
            wr32(this.wrapping_add(TASK_SUB), built);
            lf_checker_rt::callee_thiscall!(11, u32, built, 1, notify_callback, this);
        }
        tail(this, ped)
    }
});
