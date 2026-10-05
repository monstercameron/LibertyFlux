// original: 0x00bf93e0 task_handle_acquire_scaled (proposed)

/// Ensure the task handle for a type, build its parameter block, scale one
/// flag byte into it and stamp it with the current tick.
///
/// `this` carries a type id at `+0x08` and a flag byte at `+0x20`; `a` only
/// selects the null-target early exit and is otherwise unused.
///
/// Behaviour: a null target returns immediately (the original returns whatever
/// was in eax, unobservable; the proof never takes that path). Three log calls
/// run in order until one's answer equals the type id (or all three run). Two
/// gather calls fill scratch cells, then the manager object is asked for the
/// handle for (type id, 0, 0); a null handle returns 0. A matrix call takes
/// two cell pointers plus a flag word and fills a fifteen-word block, an
/// apply call takes that block, and the flag byte shifted
/// right by one is converted to float and pushed into the handle. Bit 0 of
/// the flag then selects between pushing 1.0 under one key or 0.0 under
/// another; a post call runs, and the handle's stamp slot (`+0x1d4`) gets the
/// global tick, bumped by one when the tick-check call's answer equals the
/// global compare word. The stamp is also the return value.
///
/// Original: 0x00bf93e0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00bf93e0(this: u32, a: u32) -> u32 {
    unsafe {
        const STR_LOG0: u32 = 0x00ebc748;
        const STR_LOG1: u32 = 0x00ebc75c;
        const STR_LOG2: u32 = 0x00ebc770;
        const STR_SET1: u32 = 0x00ebc784;
        const STR_SET0: u32 = 0x00ebc78c;
        const TASK_MGR: u32 = 0x01394d60;
        const TICK_CMP: u32 = 0x011f702c;
        const TICK_SRC: u32 = 0x011f70c4;
        const TYPE_ID: u32 = 0x08;
        const FLAG: u32 = 0x20;
        const HANDLE_STAMP: u32 = 0x1d4;
        const ONE_BITS: u32 = 0x3f800000;
        const C_L0: u32 = 1;
        const C_L1: u32 = 2;
        const C_L2: u32 = 3;
        const C_G1: u32 = 4;
        const C_G2: u32 = 5;
        const C_ACQUIRE: u32 = 6;
        const C_MAT: u32 = 7;
        const C_APPLY: u32 = 8;
        const C_SCALE: u32 = 9;
        const C_SET: u32 = 10;
        const C_POST: u32 = 11;
        const C_TICK2: u32 = 12;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if a == 0 {
            // Original returns incoming eax here; unobservable, never taken
            // by the proof (see narrowed). The value below is unreachable.
            return 0;
        }
        let type_id = rd32(this.wrapping_add(TYPE_ID));
        for (id, s) in [(C_L0, STR_LOG0), (C_L1, STR_LOG1), (C_L2, STR_LOG2)] {
            let r = lf_checker_rt::callee_cdecl!(id, u32, lf_checker_rt::relocated(s), 0);
            if type_id == r {
                break;
            }
        }
        let mut cell20 = [0u32; 3];
        lf_checker_rt::callee_thiscall!(C_G1, u32, this, cell20.as_mut_ptr() as u32);
        let mut cell10 = [0u32; 3];
        lf_checker_rt::callee_thiscall!(C_G2, u32, this, cell10.as_mut_ptr() as u32);
        let handle = lf_checker_rt::callee_thiscall!(
            C_ACQUIRE,
            u32,
            lf_checker_rt::relocated(TASK_MGR),
            type_id,
            0,
            0
        );
        if handle == 0 {
            return 0;
        }
        let mut cell30 = [0u32; 15];
        lf_checker_rt::callee_cdecl!(
            C_MAT,
            u32,
            cell30.as_mut_ptr() as u32,
            cell20.as_mut_ptr() as u32,
            cell10.as_mut_ptr() as u32,
            0
        );
        // The apply call takes the matrix block itself.
        lf_checker_rt::callee_thiscall!(C_APPLY, u32, handle, cell30.as_mut_ptr() as u32);
        let flag = (rd32(this.wrapping_add(FLAG)) & 0xff) as u8;
        let scaled = (flag.wrapping_shr(1) as u32) as f32;
        lf_checker_rt::callee_thiscall!(C_SCALE, u32, handle, scaled.to_bits());
        if flag & 1 != 0 {
            lf_checker_rt::callee_thiscall!(
                C_SET,
                u32,
                handle,
                lf_checker_rt::relocated(STR_SET1),
                ONE_BITS
            );
        } else {
            lf_checker_rt::callee_thiscall!(
                C_SET,
                u32,
                handle,
                lf_checker_rt::relocated(STR_SET0),
                0
            );
        }
        lf_checker_rt::callee_thiscall!(C_POST, u32, handle);
        let tick_answer = lf_checker_rt::callee_cdecl!(C_TICK2, u32,);
        let cmp = rd32(lf_checker_rt::relocated(TICK_CMP));
        let tick = rd32(lf_checker_rt::relocated(TICK_SRC));
        let stamped = if cmp != tick_answer {
            tick
        } else {
            tick.wrapping_add(1)
        };
        wr32(handle.wrapping_add(HANDLE_STAMP), stamped);
        stamped
    }
});
