// original: 0x00bee8d0 ped_task_run_mover (proposed)

/// Run one ped-task mover step: blend toward the target through the mover
/// helpers, or take the fallback helper, then finalize the task state.
///
/// `this` is the task (`+0x10` tag byte, `+0xc` word argument, `+0xe` global
/// slot index); `target` the task state, null meaning nothing to do; `t` the
/// blend factor as float bits. The tag byte is copied to `target+0x63` and a
/// wake virtual slot (`+0x18` of the target's table) runs first.
///
/// The fallback helper (taking a scratch word) runs when the target carries
/// flag `0x400` at `+0x24`, when `t` is zero, or when the readiness callee
/// answers `0xffffff`. Otherwise, when the readiness callee answers nonzero
/// on its second poll, the blend runs: a probe callee fills a two-word
/// struct, the sum of its words addresses the combine callee (with two
/// scratch buffers and the struct), and the pose-blend callee mixes them with
/// `t`. A NaN factor takes the blend path, matching the original's
/// `ucomiss`+`lahf` test.
///
/// The tail always runs: a finish virtual slot (`+0x4`, scratch word plus two
/// zero words); a select slot (`+0x38`) when the word at `target+0x2e` is
/// `0xffff`; byte 2 at `target+0x41`; a register callee taking the target;
/// the target stored into the global slot table at index `this+0xe`; and the
/// ambient-jet callee on the task.
///
/// Original: thiscall, two stack words (`target`, `t`), no meaningful return
/// value.
lf_checker_rt::export!(thiscall, rw_00bee8d0(this: u32, target: u32, t: u32) -> u32 {
    unsafe {
        const TAG: u32 = 0x10;
        const WORD_ARG: u32 = 0x0c;
        const SLOT_INDEX: u32 = 0x0e;
        const TARGET_TAG: u32 = 0x63;
        const TARGET_FLAGS: u32 = 0x24;
        const SNAP_FLAG: u32 = 0x400;
        const TARGET_SEL: u32 = 0x2e;
        const TARGET_DONE: u32 = 0x41;
        const SELECTED: u16 = 0xffff;
        const DONE_MARK: u8 = 2;
        const SLOT_TABLE: u32 = 0x169e7a0;
        const NOT_READY: u32 = 0xffffff;
        const VTABLE_WAKE: u32 = 0x18;
        const VTABLE_FINISH: u32 = 0x04;
        const VTABLE_SELECT: u32 = 0x38;
        const CALLEE_READY: u32 = 2;
        const CALLEE_PROBE: u32 = 3;
        const CALLEE_COMBINE: u32 = 4;
        const CALLEE_BLEND: u32 = 5;
        const CALLEE_FALLBACK: u32 = 6;
        const CALLEE_REGISTER: u32 = 9;
        const CALLEE_JET: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        if target == 0 {
            return 0;
        }
        wr8(target + TARGET_TAG, rd8(this + TAG));
        let wake: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(target) + VTABLE_WAKE) as usize);
        wake(target);

        // `tt == 0.0` is false for NaN, so NaN takes the blend path.
        let tt = f32::from_bits(t);
        let fallback = rd32(target + TARGET_FLAGS) & SNAP_FLAG != 0
            || tt == 0.0
            || lf_checker_rt::callee_thiscall!(CALLEE_READY, u32, this) == NOT_READY;
        if !fallback && lf_checker_rt::callee_thiscall!(CALLEE_READY, u32, this) != 0 {
            let mut probe = [0u32; 3];
            lf_checker_rt::callee_cdecl!(CALLEE_PROBE, u32, probe.as_mut_ptr() as u32, this);
            let combined = probe[0].wrapping_add(probe[1]);
            let dir = [0u32; 4];
            let out = [0u32; 8];
            lf_checker_rt::callee_thiscall!(
                CALLEE_COMBINE, u32, combined, dir.as_ptr() as u32, probe.as_ptr() as u32
            );
            lf_checker_rt::callee_thiscall!(
                CALLEE_BLEND, u32, this, out.as_ptr() as u32, dir.as_ptr() as u32,
                probe.as_ptr() as u32, t
            );
        } else if fallback {
            let scratch = [0u32; 4];
            lf_checker_rt::callee_thiscall!(CALLEE_FALLBACK, u32, this, scratch.as_ptr() as u32);
        }

        let finish: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(target) + VTABLE_FINISH) as usize);
        let tmp = [0u32; 4];
        finish(target, tmp.as_ptr() as u32, 0, 0);
        if rd16(target + TARGET_SEL) == SELECTED {
            let select: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(target) + VTABLE_SELECT) as usize);
            select(target, rd16(this + WORD_ARG) as u32);
        }
        wr8(target + TARGET_DONE, DONE_MARK);
        lf_checker_rt::callee_cdecl!(CALLEE_REGISTER, u32, target, 0);
        let slot = SLOT_TABLE + (rd8(this + SLOT_INDEX) as u32) * 4;
        *lf_checker_rt::global::<u32>(slot) = target;
        lf_checker_rt::callee_thiscall!(CALLEE_JET, u32, this);
        0
    }
});
