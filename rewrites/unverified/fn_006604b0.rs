// original: 0x006604B0 snAddRemoteGamer_probe (proposed)

/// Probe whether a remote gamer is already known, registering it if not.
///
/// `this` is the task and `arg` a gamer descriptor (`a0` is never read).
/// The notifier at `this+0x98` sees the descriptor (callee 1), then the
/// session pair `this+0xd8`/`+0xdc` is resolved (callee 2). On a hit the
/// matcher (callee 3) runs on the manager's `+0x24` object with the
/// session's `+0x48` block and the descriptor's `+0x78` block, the state
/// at `this+0x94` becomes 2 and the function returns 1.
///
/// On a miss a 14-byte key (words from the descriptor's `+0x78` block:
/// u32, u16, u32, u16) is built in a scratch struct and identified
/// (callee 4) together with the descriptor's `+0x8` block; the registrar
/// (callee 5) then sees the manager's `+0x118` word and `this+0x530` on
/// the `this+0x538` object. A zero answer returns the scratch byte the
/// original reads (zero under the contract's zero stack fill); otherwise
/// the `this+0x538` object's virtual slot `+8` runs (callee 6), the
/// attacher (callee 7) runs on the manager's `+0x32e0` object with a
/// scratch word, zero and the `this+0x538` object, the state becomes 0
/// and the function returns 1.
///
/// Original: 0x006604B0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_006604b0(this: u32, _a0: u32, arg: u32) -> u32 {
    unsafe {
        const NOTIFY: u32 = 0x98;
        const MGR: u32 = 0x60;
        const SESS0: u32 = 0xd8;
        const SESS1: u32 = 0xdc;
        const STATE: u32 = 0x94;
        const C_NOTIFY: u32 = 1;
        const C_SESS: u32 = 2;
        const C_MATCH: u32 = 3;
        const C_IDENT: u32 = 4;
        const C_REG: u32 = 5;
        const C_VT8: u32 = 6;
        const C_ATTACH: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let _: u32 =
            lf_checker_rt::callee_thiscall!(C_NOTIFY, u32, this.wrapping_add(NOTIFY), arg);
        let mgr = rd32(this.wrapping_add(MGR));
        let sess: u32 = lf_checker_rt::callee_thiscall!(
            C_SESS,
            u32,
            mgr,
            rd32(this.wrapping_add(SESS0)),
            rd32(this.wrapping_add(SESS1))
        );
        if sess != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                C_MATCH,
                u32,
                rd32(mgr.wrapping_add(0x24)),
                sess.wrapping_add(0x48),
                arg.wrapping_add(0x78)
            );
            wr32(this.wrapping_add(STATE), 2);
            return 1;
        }
        let mut key = [0u32; 4];
        key[0] = rd32(arg.wrapping_add(0x78));
        (key.as_mut_ptr() as *mut u16).add(2).write_unaligned(rd16(arg.wrapping_add(0x7c)));
        key[2] = rd32(arg.wrapping_add(0x80));
        (key.as_mut_ptr() as *mut u16).add(6).write_unaligned(rd16(arg.wrapping_add(0x84)));
        let mgr2 = rd32(this.wrapping_add(MGR));
        let _: u32 = lf_checker_rt::callee_thiscall!(
            C_IDENT,
            u32,
            key.as_mut_ptr() as u32,
            arg.wrapping_add(8)
        );
        let registered: u32 = lf_checker_rt::callee_thiscall!(
            C_REG,
            u32,
            this.wrapping_add(0x538),
            rd32(mgr2.wrapping_add(0x118)),
            this.wrapping_add(0x530)
        );
        if registered & 0xff == 0 {
            return 0;
        }
        let obj = this.wrapping_add(0x538);
        let slot = rd32(rd32(obj).wrapping_add(8));
        let f: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        let _ = C_VT8;
        let _ = f(obj);
        let mut scratch = [0u32; 1];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            C_ATTACH,
            u32,
            mgr2.wrapping_add(0x32e0),
            scratch.as_mut_ptr() as u32,
            0,
            obj
        );
        wr32(this.wrapping_add(STATE), 0);
        1
    }
});
