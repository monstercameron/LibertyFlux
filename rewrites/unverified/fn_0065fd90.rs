// original: 0x0065FD90 rage::snHandleJoinRequestTask::vf7

/// Run one step of the join-request task, then tear it down.
///
/// `this` is the task, `a0` selects the path (`a1` is never read). On the
/// `a0 == 1` path the manager at `this+0x60` looks up the joiner key at
/// `this+0xd8` (callee 1); a miss ends the step early. On a hit the
/// session pair `this+0xd0`/`+0xd4` is resolved (callee 2, kept for the
/// later send), the lookup runs again and marks the record (byte
/// `+0x80`, bit 0), and, unless `this+0x528` is set, the registrar
/// (callee 3) sees `this+0x98` with the manager's `+0x32b0` word and
/// `+0x2eb0` block. The join worker (callee 4) then sees `this+0x98`,
/// `this+0x328` and `this+0x320`. If the session resolved and `+0x528`
/// is still clear, the sender (callee 5) is called on the manager's
/// `+0xc6c` object with the session's first word and the manager's
/// `+0x540`/`+0x544` words.
///
/// Both paths then race a one-shot box at `this+8` with an interlocked
/// compare-exchange (callee 6: expect 1, set 3 on the `a0 == 1` path, 2
/// otherwise); on winning (answer 1) the box's second word is cleared.
/// The atomic's own update of the box is the import's doing and is not
/// part of the stubbed comparison on either side. Finally, when both
/// `this+0x18` and `this+8` are nonzero, the teardown helper (callee 7)
/// runs on `this+0x18 + 0x10` with a scratch word and the task, and the
/// task is closed (`+0x60` and `+0x8` cleared, `+0xc` set to 2).
///
/// Original: 0x0065FD90 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0065fd90(this: u32, a0: u32, _a1: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x60;
        const KEY: u32 = 0xd8;
        const SESS0: u32 = 0xd0;
        const SESS1: u32 = 0xd4;
        const FLAG528: u32 = 0x528;
        const REG_OBJ: u32 = 0x98;
        const JOIN_A: u32 = 0x328;
        const JOIN_B: u32 = 0x320;
        const BOX: u32 = 0x08;
        const AUX: u32 = 0x18;
        const C_LOOKUP: u32 = 1;
        const C_SESS: u32 = 2;
        const C_REG: u32 = 3;
        const C_JOIN: u32 = 4;
        const C_SEND: u32 = 5;
        const C_CMPXCHG: u32 = 6;
        const C_AUX: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let new_value: u32;
        if a0 == 1 {
            new_value = 3;
            let mgr = rd32(this.wrapping_add(MGR));
            let found: u32 =
                lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, mgr, this.wrapping_add(KEY));
            if found != 0 {
                let sess: u32 = lf_checker_rt::callee_thiscall!(
                    C_SESS,
                    u32,
                    mgr,
                    rd32(this.wrapping_add(SESS0)),
                    rd32(this.wrapping_add(SESS1))
                );
                let rec: u32 =
                    lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, mgr, this.wrapping_add(KEY));
                let mark = rec.wrapping_add(0x80) as *mut u8;
                mark.write(mark.read() | 1);
                if rd8(this.wrapping_add(FLAG528)) == 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        C_REG,
                        u32,
                        mgr,
                        this.wrapping_add(REG_OBJ),
                        rd32(mgr.wrapping_add(0x32b0)),
                        mgr.wrapping_add(0x2eb0)
                    );
                }
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    C_JOIN,
                    u32,
                    mgr,
                    this.wrapping_add(REG_OBJ),
                    this.wrapping_add(JOIN_A),
                    rd32(this.wrapping_add(JOIN_B))
                );
                if sess != 0 && rd8(this.wrapping_add(FLAG528)) == 0 {
                    let mut words = [
                        rd32(mgr.wrapping_add(0x540)),
                        rd32(mgr.wrapping_add(0x544)),
                    ];
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        C_SEND,
                        u32,
                        mgr.wrapping_add(0xc6c),
                        rd32(sess),
                        words.as_mut_ptr() as u32
                    );
                }
            }
        } else {
            new_value = 2;
        }
        let boxp = rd32(this.wrapping_add(BOX));
        if boxp != 0 {
            let won: u32 =
                lf_checker_rt::callee_stdcall!(C_CMPXCHG, u32, boxp, new_value, 1);
            if won == 1 {
                wr32(boxp.wrapping_add(4), 0);
            }
        }
        let aux = rd32(this.wrapping_add(AUX));
        if aux != 0 && rd32(this.wrapping_add(BOX)) != 0 {
            let mut scratch = [0u32; 1];
            let _: u32 = lf_checker_rt::callee_thiscall!(
                C_AUX,
                u32,
                aux.wrapping_add(0x10),
                scratch.as_mut_ptr() as u32,
                this
            );
        }
        wr32(this.wrapping_add(MGR), 0);
        wr32(this.wrapping_add(0x0c), 2);
        wr32(this.wrapping_add(BOX), 0);
        0
    }
});
