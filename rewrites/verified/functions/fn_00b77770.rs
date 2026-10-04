// original: 0x00b77770 task_slot_reassign (proposed)

/// Reassign this owner's slot: look up the task matching a salted key and
/// either attach it or, when nothing matches, detach every slotted task.
///
/// `this` points to the owner (slots at `+0x04`, nine of them; key field at
/// `+0x1a0`, selected index at `+0x1a4`, state at `+0x19c`). `arg0` goes to
/// the gate callee, which runs with a fixed global object as its `this`; a
/// zero answer returns immediately with that answer.
///
/// Otherwise the key is `key + BIAS + SALT` (wrapping) divided by the
/// countdown callee's answer, and the remainder selects a lookup. A null
/// result records the remainder at `+0x1a8` and detaches every slot except
/// the selected index: each live object is released through virtual slot
/// `VT_RELEASE` with argument `0xff` and its slot is cleared. A result equal
/// to `this` detaches nothing. Any other result is attached through callee 4
/// with (slot object, 1), then the slot object is released through
/// `VT_RELEASE` with the remainder, and the slot is cleared.
///
/// Both globals are cleared on every taken path. Returns 1 after an attach
/// or a self-match (state 2, index reset to 9) and 0 after a full detach
/// (state 5); only the low byte is significant.
///
/// Original: 0x00b77770 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b77770(this: u32, arg0: u32) -> u32 {
    unsafe {
        const GATE_THIS: u32 = 0x012845d0;
        const G_SALT: u32 = 0x012845c4;
        const G_BIAS: u32 = 0x0167ccc0;
        const SLOT_BASE: u32 = 0x04;
        const SLOT_COUNT: u32 = 9;
        const KEY_OFF: u32 = 0x1a0;
        const INDEX_OFF: u32 = 0x1a4;
        const STATE_OFF: u32 = 0x19c;
        const REM_OFF: u32 = 0x1a8;
        const VT_RELEASE: u32 = 0x24;
        const RELEASE_ALL: u32 = 0xff;
        const GATE_CALLEE: u32 = 1;
        const COUNT_CALLEE: u32 = 2;
        const LOOKUP_CALLEE: u32 = 3;
        const ATTACH_CALLEE: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn release(obj: u32, arg: u32) {
            unsafe {
                let slot = rd32(rd32(obj).wrapping_add(VT_RELEASE));
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(obj, arg);
            }
        }

        let gate: u32 = lf_checker_rt::callee_thiscall!(
            GATE_CALLEE,
            u32,
            lf_checker_rt::relocated(GATE_THIS),
            arg0
        );
        if gate & 0xff == 0 {
            return gate;
        }

        let salt = rd32(lf_checker_rt::relocated(G_SALT));
        let bias = rd32(lf_checker_rt::relocated(G_BIAS));
        let divisor: u32 = lf_checker_rt::callee_cdecl!(COUNT_CALLEE, u32,);
        let key = rd32(this.wrapping_add(KEY_OFF))
            .wrapping_add(bias)
            .wrapping_add(salt);
        let rem = key % divisor;
        let found: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, rem);

        if found == 0 {
            wr32(this.wrapping_add(REM_OFF), rem);
            let skip = rd32(this.wrapping_add(INDEX_OFF));
            let mut i = 0u32;
            while i < SLOT_COUNT {
                if i != skip {
                    let slot = this
                        .wrapping_add(SLOT_BASE)
                        .wrapping_add(i.wrapping_mul(4));
                    let obj = rd32(slot);
                    if obj != 0 {
                        release(obj, RELEASE_ALL);
                        wr32(slot, 0);
                    }
                }
                i += 1;
            }
            wr32(lf_checker_rt::relocated(G_SALT), 0);
            wr32(lf_checker_rt::relocated(G_BIAS), 0);
            wr32(this.wrapping_add(STATE_OFF), 5);
            return 0;
        }

        if found != this {
            let idx = rd32(this.wrapping_add(INDEX_OFF));
            let slot = this
                .wrapping_add(SLOT_BASE)
                .wrapping_add(idx.wrapping_mul(4));
            let obj = rd32(slot);
            if obj != 0 {
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(ATTACH_CALLEE, u32, found, obj, 1);
                release(obj, rem);
                wr32(slot, 0);
            }
        }
        wr32(lf_checker_rt::relocated(G_SALT), 0);
        wr32(lf_checker_rt::relocated(G_BIAS), 0);
        wr32(this.wrapping_add(STATE_OFF), 2);
        wr32(this.wrapping_add(INDEX_OFF), SLOT_COUNT);
        1
    }
});
