// original: 0x00bf8800 task_handle_acquire_blend (proposed)

/// Ensure the task handle for a target exists, blend one parameter into it,
/// and stamp it with the current tick.
///
/// `this` is the task owner object (a dword parameter at `+0x08`, a blend
/// input float at `+0x18`, a three-float vector at `+0x1c`). `target` selects
/// the task, `blend` is a blend factor as float bits.
///
/// Behaviour: a null target returns immediately (the original returns whatever
/// happened to be in eax there, which no rewrite can observe; the proof never
/// takes that path). Otherwise a log call runs, then the manager object
/// (fixed global `TASK_MGR`) is asked for the handle for (`target`,
/// `this.+0x08`, constant 2.0, 0), writing one flag byte through an out-cell.
/// A null handle returns 0. When `blend` compares equal (ucomiss, so negative
/// zero equals positive zero and NaN is unequal) to the global threshold
/// `BLEND_THRESH`, the `+0x18` float is pushed into the handle under a fixed
/// key. A non-zero flag byte then skips the check/compute/apply trio and goes
/// straight to the vector path; a zero flag runs a check call first and the
/// compute/apply pair only when the check's low byte is non-zero (the compute
/// call writes one dword through its out-cell, which becomes apply's middle
/// argument). With a zero flag the function returns the last call's answer
/// (the set, check or apply call's return value, whatever it was). On the vector
/// path the `+0x1c` vector is pushed into the handle, a post call runs, and
/// the handle's stamp slot (`+0x1d4`) gets the global tick, bumped by one when
/// the tick-check call's answer equals the global compare word; the stamp is
/// also the return value.
///
/// Original: 0x00bf8800 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00bf8800(this: u32, target: u32, blend: u32) -> u32 {
    unsafe {
        const STR_LOG_MSG: u32 = 0x00ebc4c8;
        const STR_SET_KEY: u32 = 0x00ebc4d4;
        const TASK_MGR: u32 = 0x01394d60;
        const BLEND_THRESH: u32 = 0x00fe8628;
        const TICK_CMP: u32 = 0x011f702c;
        const TICK_SRC: u32 = 0x011f70c4;
        const PARAM8: u32 = 0x08;
        const BLEND_VAL: u32 = 0x18;
        const VEC3: u32 = 0x1c;
        const HANDLE_STAMP: u32 = 0x1d4;
        const TWO_BITS: u32 = 0x40000000;
        const C_LOG: u32 = 1;
        const C_ACQUIRE: u32 = 2;
        const C_SETVAL: u32 = 3;
        const C_CHECK: u32 = 4;
        const C_COMPUTE: u32 = 5;
        const C_APPLY: u32 = 6;
        const C_SETVEC: u32 = 7;
        const C_POST: u32 = 8;
        const C_TICK2: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if target == 0 {
            // Original returns incoming eax here; unobservable, never taken
            // by the proof (see narrowed). The value below is unreachable.
            return 0;
        }
        lf_checker_rt::callee_cdecl!(C_LOG, u32, lf_checker_rt::relocated(STR_LOG_MSG), 0);
        let mut acquired: u32 = 0;
        let param = rd32(this.wrapping_add(PARAM8));
        let handle = lf_checker_rt::callee_thiscall!(
            C_ACQUIRE,
            u32,
            lf_checker_rt::relocated(TASK_MGR),
            target,
            param,
            (&mut acquired as *mut u32) as u32,
            TWO_BITS,
            0
        );
        if handle == 0 {
            return 0;
        }
        // ucomiss-equal: -0.0 == +0.0, NaN unequal to everything. Rust `==`
        // on f32 has exactly these semantics; no arithmetic is done.
        let is_eq =
            f32::from_bits(blend) == f32::from_bits(rd32(lf_checker_rt::relocated(BLEND_THRESH)));
        if is_eq {
            let answer = lf_checker_rt::callee_thiscall!(
                C_SETVAL,
                u32,
                handle,
                lf_checker_rt::relocated(STR_SET_KEY),
                rd32(this.wrapping_add(BLEND_VAL))
            );
            if acquired & 0xff == 0 {
                return answer;
            }
        } else if acquired & 0xff == 0 {
            let ok = lf_checker_rt::callee_thiscall!(C_CHECK, u32, this);
            if ok & 0xff == 0 {
                return ok;
            }
            let mut out: u32 = 0;
            lf_checker_rt::callee_cdecl!(C_COMPUTE, u32, (&mut out as *mut u32) as u32, this);
            let answer = lf_checker_rt::callee_thiscall!(C_APPLY, u32, this, handle, out, blend);
            return answer;
        }
        let mut vec = [
            rd32(this.wrapping_add(VEC3)),
            rd32(this.wrapping_add(VEC3 + 4)),
            rd32(this.wrapping_add(VEC3 + 8)),
        ];
        lf_checker_rt::callee_thiscall!(C_SETVEC, u32, handle, vec.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(C_POST, u32, handle);
        let tick_answer = lf_checker_rt::callee_cdecl!(C_TICK2, u32,);
        let cmp = rd32(lf_checker_rt::relocated(TICK_CMP));
        let tick = rd32(lf_checker_rt::relocated(TICK_SRC));
        // Flags survive the tick load in the original, so the bump compares
        // the check answer against the compare word, not the tick.
        let stamped = if cmp != tick_answer {
            tick
        } else {
            tick.wrapping_add(1)
        };
        wr32(handle.wrapping_add(HANDLE_STAMP), stamped);
        stamped
    }
});
