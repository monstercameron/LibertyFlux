// original: 0x0097ED80 LADDER_SLIDE

/// Fire the ladder-slide audio event for a ped task object.
///
/// `this` is the task object. The call is a no-op unless the audio system
/// is up: a quit flag must be clear, two build/session words must agree,
/// and a mode word must differ from the skip value. The task's own
/// trigger word at `+0x19C` must be zero (a set word means the event
/// already fired or is suppressed).
///
/// On the main path a lazily initialised hash word (flag at `G_HASH_FLAG`,
/// value at `G_HASH`) is computed once from a name string, a scratch
/// request buffer is initialised by callee 2 and filled with the ped
/// pointer (`+0x120`, biased) at word 3 and a sub-object word (`+0x8`) at
/// word 8, callee 3 resolves a handle from the buffer, callee 4 derives a
/// parameter from the handle, and callee 5 (this = task) arbitrates: a
/// zero low byte releases the handle through callee 7, otherwise callee 6
/// posts the event with a (0, -1, 0x48) descriptor and the buffer.
///
/// Original: 0x0097ED80 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_0097ED80(this: u32) -> u32 {
    unsafe {
        const G_QUIT: u32 = 0x011F7060;
        const G_SESS_A: u32 = 0x012088B4;
        const G_SESS_B: u32 = 0x00F1C040;
        const G_MODE: u32 = 0x01037720;
        const G_HASH_FLAG: u32 = 0x01231750;
        const G_HASH: u32 = 0x0123174C;
        const SKIP_MODE: u32 = 0x12;
        const NAME_STR: u32 = 0x00E8CCE4;
        const OFF_TRIGGER: u32 = 0x19C;
        const OFF_PED: u32 = 0x120;
        const OFF_SUB: u32 = 0x08;
        const PED_BIAS: u32 = 0x780;
        const BUF_PED_WORD: usize = 3;
        const BUF_SUB_WORD: usize = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn gget(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        unsafe fn gset(va: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(va).write(v) }
        }

        if gget(G_QUIT) == 1 {
            return 0;
        }
        if gget(G_SESS_A) != gget(G_SESS_B) {
            return 0;
        }
        if gget(G_MODE) == SKIP_MODE {
            return 0;
        }
        let trigger = this.wrapping_add(OFF_TRIGGER);
        if rd32(trigger) != 0 {
            return 0;
        }
        let flag = gget(G_HASH_FLAG);
        if flag & 1 == 0 {
            gset(G_HASH_FLAG, flag | 1);
            let h: u32 = lf_checker_rt::callee_cdecl!(
                1,
                u32,
                lf_checker_rt::relocated(NAME_STR),
                0
            );
            gset(G_HASH, h);
        }
        let mut buf = [0u32; 16];
        let buf_ptr = buf.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, buf_ptr);
        buf[BUF_PED_WORD] = rd32(this.wrapping_add(OFF_PED)).wrapping_add(PED_BIAS);
        buf[BUF_SUB_WORD] = rd32(this.wrapping_add(OFF_SUB));
        let handle: u32 = lf_checker_rt::callee_thiscall!(3, u32, buf_ptr);
        let param: u32 = lf_checker_rt::callee_cdecl!(4, u32, handle);
        let hash = gget(G_HASH);
        let arb: u32 =
            lf_checker_rt::callee_thiscall!(5, u32, this, hash, trigger, buf_ptr, handle, param, 0);
        if (arb & 0xFF) == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, handle);
            return 0;
        }
        let mut desc = [0u32, 0xFFFF_FFFF, 0x48];
        let desc_ptr = desc.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            6,
            u32,
            hash,
            0,
            1,
            1,
            buf_ptr,
            desc_ptr,
            rd32(this.wrapping_add(OFF_PED)),
            handle
        );
        0
    }
});
