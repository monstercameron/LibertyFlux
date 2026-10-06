// original: 0x00cce2f0 CTaskSimpleDead::vf17

/// Dead-task tick: mark the ped dead, publish a dead-ped event, pick the
/// follow-up task. Returns 0 in AL (upper bytes are callee leftovers).
///
/// `task` points to the task (`+0x14` state word, `+0x18` flag byte);
/// `ped` is the ped (`+0x20`/`+0x6c`/`+0x78` object links, `+0x2c` word,
/// `+0x218`/`+0x219`/`+0x26c`/`+0xf4` bytes, `+0x7b8`/`+0xba0`/`+0xbe0`
/// words, vtable at `+0x0`). Original is thiscall: ecx = task, one stack
/// word = ped, callee pops 4.
///
/// The tick sets bits 2-3 of the ped status word (`+0xbe0`), then records a
/// death pose id at `+0xba0`: 0x1bc when the ped is flagged dying but not
/// dead (`+0x218` zero, `+0x219` nonzero), otherwise 0x1bb when the scaled
/// word (`+0x2c` as float times the 3.05e-5 factor) compares equal to zero
/// and 0x1ba when it does not (the original tests this through
/// `ucomiss`/`lahf`, which reports equal only for an ordered zero, the same
/// as `== 0.0`). It then sets bit 7 of `+0xf4`.
///
/// When flag bit 0 of the task is set, the tick runs the death branch:
/// notify the ped (callee 1, arg 2), report through the reporter (callee 2,
/// cdecl, six words: 0xe, `[ped+0x20]+0x30`, ped, 0, 0, 0), release the ped
/// through virtual slot `+0xf4` with args 0, 0 (callee 3) unless an
/// auxiliary object exists whose byte at `+0xe` reads nonzero, join the session
/// (callee 4, cdecl, ped), drive the ped's block at `+0x2b0` (callee 5,
/// args 0, 0, 1, 0), construct a dead-ped event in a stack slot (callee 6,
/// thiscall on the slot with ped, the task's bit 1 and the task state
/// word), fetch the event singleton (callee 7, cdecl, no words: it pops
/// nothing, and the three words pushed before it belong to the next call,
/// which pops them), offer the event to it (callee 8, thiscall on the
/// singleton with the slot, 0, 1) and destroy the event (callee 9).
///
/// Liveness is then re-read: when the ped state word (`+0x7b8`) reads 6 the
/// task flag bit 3 is set, otherwise the ped's float block (callee 10,
/// thiscall on `[ped+0x78]`, called up to twice) is compared against 0.99
/// and 0.0 (both ordered comparisons, so NaN clears the bit) and bit 3 is
/// set only when the first float exceeds 0.99 and the second is not below
/// zero. Bit 3 selects the follow-up task (callee 11 vs 12, thiscall with
/// the ped); when bit 1 is clear and `[ped+0x26c]` bit 2 is clear a third
/// task runs (callee 13). Bit 0 is cleared and 0 returned.
///
/// Proving notes: the event slot address is frame-local, so it is skipped
/// wherever passed (callees 6 and 9 register, callee 8 argument 0) and the
/// slot's first four words are snapshotted instead; callee 6's scripted
/// writes stand in for the constructor's stores (the values the real
/// constructor stores all arrive as its compared arguments, so nothing
/// input-dependent is lost). The return channel is AL only.
lf_checker_rt::export!(thiscall, rw_00cce2f0(task: u32, ped: u32) -> u8 {
    unsafe {
        const TASK_STATE: u32 = 0x14;
        const TASK_FLAGS: u32 = 0x18;
        const PED_LINK: u32 = 0x20;
        const PED_WORD: u32 = 0x2c;
        const PED_AUX: u32 = 0x6c;
        const PED_MGR: u32 = 0x78;
        const PED_MARK: u32 = 0xf4;
        const PED_DYING: u32 = 0x218;
        const PED_DEAD: u32 = 0x219;
        const PED_COND: u32 = 0x26c;
        const PED_BLOCK: u32 = 0x2b0;
        const PED_ALIVE: u32 = 0x7b8;
        const PED_POSE: u32 = 0xba0;
        const PED_STATUS: u32 = 0xbe0;
        const AUX_GATE: u32 = 0xe;
        const FLOAT_A: u32 = 0x48;
        const FLOAT_B: u32 = 0x4c;
        const SCALE_BITS: u32 = 0x00fe8684;
        const LIMIT_BITS: u32 = 0x00fe88d4;
        const SLOT_RELEASE: u32 = 0xf4;
        const POSE_DYING: u32 = 0x1bc;
        const POSE_ZERO: u32 = 0x1bb;
        const POSE_SCALED: u32 = 0x1ba;
        const ALIVE_READY: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        /// Virtual call with two stack arguments through the object's table.
        #[inline(always)]
        unsafe fn vcall2(obj: u32, slot: u32, a: u32, b: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                f(obj, a, b)
            }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        wr32(
            ped.wrapping_add(PED_STATUS),
            rd32(ped.wrapping_add(PED_STATUS)) | 0xc,
        );
        if rd8(ped.wrapping_add(PED_DYING)) != 0 || rd8(ped.wrapping_add(PED_DEAD)) == 0 {
            let w = rd16(ped.wrapping_add(PED_WORD));
            let scale = f32::from_bits(rd32(lf_checker_rt::relocated(SCALE_BITS)));
            let f = fmul(w as f32, scale);
            wr32(
                ped.wrapping_add(PED_POSE),
                if f == 0.0 { POSE_ZERO } else { POSE_SCALED },
            );
        } else {
            wr32(ped.wrapping_add(PED_POSE), POSE_DYING);
        }
        wr8(
            ped.wrapping_add(PED_MARK),
            rd8(ped.wrapping_add(PED_MARK)) | 0x80,
        );
        let mut flags = rd8(task.wrapping_add(TASK_FLAGS));
        if flags & 1 != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(1u32, u32, ped, 2u32);
            let link = rd32(ped.wrapping_add(PED_LINK)).wrapping_add(0x30);
            let _: u32 = lf_checker_rt::callee_cdecl!(2u32, u32, 0xeu32, link, ped, 0u32, 0u32, 0u32);
            let aux = rd32(ped.wrapping_add(PED_AUX));
            if aux == 0 || rd8(aux.wrapping_add(AUX_GATE)) == 0 {
                let _: u32 = vcall2(ped, SLOT_RELEASE, 0u32, 0u32);
            }
            let _: u32 = lf_checker_rt::callee_cdecl!(4u32, u32, ped);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                5u32, u32, ped.wrapping_add(PED_BLOCK), 0u32, 0u32, 1u32, 0u32
            );
            let bit = u32::from((rd8(task.wrapping_add(TASK_FLAGS)) >> 1) & 1);
            let state = rd32(task.wrapping_add(TASK_STATE));
            let mut ev = [0u32; 4];
            let slot = ev.as_mut_ptr() as u32;
            let _: u32 = lf_checker_rt::callee_thiscall!(6u32, u32, slot, ped, bit, state);
            let singleton: u32 = lf_checker_rt::callee_cdecl!(7u32, u32,);
            let _: u32 =
                lf_checker_rt::callee_thiscall!(8u32, u32, singleton, slot, 0u32, 1u32);
            let _: u32 = lf_checker_rt::callee_thiscall!(9u32, u32, slot);
            flags = rd8(task.wrapping_add(TASK_FLAGS));
        }
        if rd32(ped.wrapping_add(PED_ALIVE)) == ALIVE_READY {
            flags |= 8;
            wr8(task.wrapping_add(TASK_FLAGS), flags);
        } else {
            let mgr = rd32(ped.wrapping_add(PED_MGR));
            let blk: u32 = lf_checker_rt::callee_thiscall!(10u32, u32, mgr);
            let limit = f32::from_bits(rd32(lf_checker_rt::relocated(LIMIT_BITS)));
            let fa = f32::from_bits(rd32(blk.wrapping_add(FLOAT_A)));
            if fa > limit {
                let blk2: u32 = lf_checker_rt::callee_thiscall!(10u32, u32, mgr);
                let fb = f32::from_bits(rd32(blk2.wrapping_add(FLOAT_B)));
                if fb >= 0.0 {
                    flags |= 8;
                    wr8(task.wrapping_add(TASK_FLAGS), flags);
                } else {
                    flags &= !8;
                    wr8(task.wrapping_add(TASK_FLAGS), flags);
                }
            } else {
                flags &= !8;
                wr8(task.wrapping_add(TASK_FLAGS), flags);
            }
        }
        if rd8(task.wrapping_add(TASK_FLAGS)) & 8 != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(11u32, u32, task, ped);
        } else {
            let _: u32 = lf_checker_rt::callee_thiscall!(12u32, u32, task, ped);
        }
        if rd8(task.wrapping_add(TASK_FLAGS)) & 2 == 0
            && rd8(ped.wrapping_add(PED_COND)) & 4 == 0
        {
            let _: u32 = lf_checker_rt::callee_thiscall!(13u32, u32, task, ped);
        }
        wr8(
            task.wrapping_add(TASK_FLAGS),
            rd8(task.wrapping_add(TASK_FLAGS)) & !1,
        );
        0
    }
});
