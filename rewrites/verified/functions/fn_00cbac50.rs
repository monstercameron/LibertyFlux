// original: 0x00cbac50 slide_task_configure_motion (proposed)

/// Configure a slide task's motion from a polled helper and trig tables.
///
/// `this` is the task object (thiscall), `task` a sibling task record, and
/// `fc`/`fd` two float arguments passed as raw bits. Returns a 32-bit handle.
///
/// The helper object at `task + HELPER` (+0xa80) is polled through its
/// virtual slot 0x44 (thiscall, no stack args). If the verdict has bit 0x800
/// set, `fd` is tripled (by the image constant 3.0); the scaled value is
/// kept for the final call.
///
/// When flag bit 0x10 in `this + FLAGS` (+0xb0) is set, the angle at
/// `this + ANGLE` (+0x28) is run through the sine/cosine pair (callees 2 and
/// 3, which take their argument in XMM0 and answer in XMM0); the sine is
/// negated. The pair feeds a 4-argument float helper (callee 4), whose
/// result passes through a 1-argument float helper (callee 5) into
/// `this + OUT` (+0x2c), and the 0x10 flag is cleared. Otherwise the same
/// pair runs on `fc`. Either way a triple (negated first result, second
/// result, 0) is staged for the final call.
///
/// The helper is polled again; its verdict OR'd with 0xd2 plus flag bit 1
/// (when `FLAGS & 2`), bit 2 (when `this + MODE` (+0x24) is not 1) and bit
/// 20 forms the mode word. A 4-argument setup call (callee 6) takes
/// (`task`, 20.0, out-buffer, 4) and fills 8 words; the final 10-argument
/// call (callee 7) takes (anchor pointer twice, triple pointer, scaled `fd`,
/// mode, 0, setup result, out pointer, task, 0). Its result is stored to
/// `this + HANDLE` (+0x44) and returned, flag bit 3 is set, and the CRT
/// stack-cookie check (callee 8, no stack args, preserves all registers) is
/// mirrored so the call log matches.
///
/// Float operation order matches the original; multiplies use the
/// keep-both-operands-live form so the backend cannot swap NaN winners.
///
/// Original: 0x00cbac50 (thiscall, three stack words, the callee pops 0xc bytes).
lf_checker_rt::export!(thiscall, rw_00cbac50(this: u32, task: u32, fc_b: u32, fd_b: u32) -> u32 {
    unsafe {
        const HELPER: u32 = 0xa80;
        const HELPER_SLOT: u32 = 0x44;
        const FLAGS: u32 = 0xb0;
        const FLAG_TRIG: u32 = 0x10;
        const FLAG_BIT2: u32 = 0x02;
        const FLAG_DONE: u32 = 0x08;
        const ANGLE: u32 = 0x28;
        const OUT: u32 = 0x2c;
        const MODE: u32 = 0x24;
        const HANDLE: u32 = 0x44;
        const ANCHOR_OFF: u32 = 0x20;
        const SCALE_ADDR: u32 = 0x00fe8a94;
        const SIGN_ADDR: u32 = 0x00fe8fa0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            let r = core::hint::black_box(a) * core::hint::black_box(b);
            core::hint::black_box(a);
            core::hint::black_box(b);
            r
        }
        #[inline(always)]
        unsafe fn poll(obj: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let slot = rd32(vt + HELPER_SLOT);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(obj)
            }
        }

        let fc = f32::from_bits(fc_b);
        let fd = f32::from_bits(fd_b);
        let obj = rd32(task + HELPER);
        let r1 = poll(obj);
        let scale = f32::from_bits(rd32(lf_checker_rt::relocated(SCALE_ADDR)));
        let sign = rd32(lf_checker_rt::relocated(SIGN_ADDR));
        let mut scal = fd;
        if r1 & 0x800 != 0 {
            scal = fmul(fd, scale);
        }
        let flags0 = rd32(this + FLAGS);
        let (t0, t1);
        if flags0 & FLAG_TRIG != 0 {
            let ang = f32::from_bits(rd32(this + ANGLE));
            let f1 = f32::from_bits(lf_checker_rt::callee_cdecl!(2, u32, ang.to_bits()));
            let neg = f32::from_bits(f1.to_bits() ^ sign);
            let f2 = f32::from_bits(lf_checker_rt::callee_cdecl!(3, u32, ang.to_bits()));
            let t = lf_checker_rt::callee_cdecl!(4, f32, neg.to_bits(), f2.to_bits(), 0u32, 0u32);
            wr32(this + OUT, t.to_bits());
            let r5 = lf_checker_rt::callee_cdecl!(5, f32, t.to_bits());
            wr32(this + OUT, r5.to_bits());
            wr32(this + FLAGS, flags0 & !FLAG_TRIG);
            t0 = neg;
            t1 = f2;
        } else {
            let f1 = f32::from_bits(lf_checker_rt::callee_cdecl!(2, u32, fc.to_bits()));
            let neg = f32::from_bits(f1.to_bits() ^ sign);
            let f2 = f32::from_bits(lf_checker_rt::callee_cdecl!(3, u32, fc.to_bits()));
            t0 = neg;
            t1 = f2;
        }
        let r2 = poll(obj);
        let mut mode = r2 | 0xd2;
        if rd32(this + FLAGS) & FLAG_BIT2 != 0 {
            mode |= 1;
        }
        if rd32(this + MODE) != 1 {
            mode |= 4;
        } else {
            mode &= !4;
        }
        mode |= 0x100000;
        let mut triple = [t0.to_bits(), t1.to_bits(), 0u32];
        let mut out = [0u32; 8];
        let r6 = lf_checker_rt::callee_cdecl!(
            6, u32, task, 0x41a00000u32, out.as_mut_ptr() as u32, 4u32
        );
        let anchor = rd32(task + ANCHOR_OFF).wrapping_add(0x30);
        let r7 = lf_checker_rt::callee_cdecl!(
            7, u32,
            anchor, anchor, triple.as_mut_ptr() as u32, scal.to_bits(), mode,
            0u32, r6, out.as_mut_ptr() as u32, task, 0u32
        );
        wr32(this + HANDLE, r7);
        wr32(this + FLAGS, rd32(this + FLAGS) | FLAG_DONE);
        lf_checker_rt::callee_cdecl!(8, u32,);
        r7
    }
});
