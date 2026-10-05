// original: 0x00cb0cf0 CTaskComplexMoveGoToPointAndStandStill::vf20 (symbols)

/// Step a move-go-to-point-and-stand-still complex task, returning the subtask.
///
/// `this` is the task, `p` the owner object. Returns a full 32-bit value in
/// EAX: the subtask kept at `this+0x8` on most paths, or the answers of
/// callees 3 and 5 on the replacement paths (thiscall: `this` in ECX, one
/// stack word, callee pops 4).
///
/// Layout read: the subtask at `this+0x8` (vtable at `+0`: slot `+0xC` is
/// the status poll, slot `+0x14` the start gate; flag dword at `+0xC` with
/// bit 0 "started" and bit 1 set after starting; object at `+0x14` whose
/// word at `+0x4` is a direct code pointer), the task vtable at `+0` (slot
/// `+0x4C` yields the replacement subtask), a progress float at `this+0x18`,
/// a parameter float at `this+0x38`, and a mode byte at `this+0x3C` (bit 2
/// selects the warm path). `p` carries four words at `+0xE0..+0xEC` used on
/// the 0x11A path. Only read-only float constants are used (3.0, 2.0, 1.0);
/// 10.0 and a large float arrive as immediates.
///
/// Algorithm: with bit 2 set, a first poll must answer exactly 0x384 or
/// control falls into the main poll: the start gate runs unless started (a
/// zero answer returns the subtask, otherwise bit 1 is set) and the fetch
/// result is returned. The main poll has three outcomes: 0x11A zeroes two
/// words of `p`, copies the third back onto itself and stores a frame word
/// to the fourth, then returns the subtask; anything but 0x384 returns the
/// subtask; 0x384 dispatches on the progress float. A progress of exactly
/// 3.0 calls callee 4 with (subtask, owner, parameter, 10.0); below 1.0
/// (or NaN) the direct slot runs with (progress bits, 0) and the subtask is
/// returned; anything else calls callee 4 with the large immediate. When
/// callee 4 accepts (AL nonzero) the subtask is returned, else the gate runs
/// unless started (zero returns the subtask, otherwise bit 1 is set) and
/// callee 5's answer for (0x11A, owner) is returned.
///
/// Edge cases: only AL of callees 2 and 4 is tested; the 3.0 test uses the
/// `ucomiss`/`lahf`/parity idiom (equal means ordered-equal, everything else
/// including NaN takes the dispatch); the 2.0 test is ordered `>=` (NaN
/// falls through) and the 1.0 test rejects on unordered too. The two polls
/// share one stub without a per-call sequence: the first poll answering
/// 0x384 always returns before the second, so both never need distinct
/// answers in one trial, and the gate likewise never fires twice in one
/// trial. The frame word stored on the 0x11A path is uninitialized stack on
/// every path that reaches it (no store precedes the read); the contract
/// defines that fill as 0 and the rewrite stores 0, which is the only
/// narrowed point of this proof. No call argument is skipped and ECX is
/// compared on every callee.
lf_checker_rt::export!(thiscall, rw_00cb0cf0(this: u32, p: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 0x08;
        const SUB_FLAGS: u32 = 0x0C;
        const STARTED_BIT: u32 = 0x01;
        const SET_BIT: u32 = 0x02;
        const DIRECT_OBJ: u32 = 0x14;
        const POLL_SLOT: u32 = 0x0C;
        const GATE_SLOT: u32 = 0x14;
        const DIRECT_SLOT: u32 = 0x04;
        const FETCH_SLOT: u32 = 0x4C;
        const MODE_OFF: u32 = 0x3C;
        const WARM_BIT: u8 = 0x04;
        const PROG_OFF: u32 = 0x18;
        const PARAM_OFF: u32 = 0x38;
        const POLL_HOT: u32 = 0x384;
        const POLL_DONE: u32 = 0x11A;
        const C_THREE: u32 = 0x00FE8A94;
        const C_TWO: u32 = 0x00FE8A24;
        const C_ONE: u32 = 0x00FE88E8;
        const K_TEN: u32 = 0x41200000;
        const K_BIG: u32 = 0x4CBebc20;
        const P_Z0: u32 = 0xE0;
        const P_Z1: u32 = 0xE4;
        const P_COPY: u32 = 0xE8;
        const P_SLOT: u32 = 0xEC;

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
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn poll(sub: u32) -> u32 {
            unsafe {
                let vt = rd32(sub);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt + POLL_SLOT) as usize);
                f(sub)
            }
        }
        #[inline(always)]
        unsafe fn gate(sub: u32, a0: u32, a1: u32, a2: u32) -> u32 {
            unsafe {
                let vt = rd32(sub);
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt + GATE_SLOT) as usize);
                f(sub, a0, a1, a2)
            }
        }

        let sub = rd32(this + SUB_OFF);
        if (rd8(this + MODE_OFF) & WARM_BIT) != 0 && poll(sub) == POLL_HOT {
            if (rd32(sub + SUB_FLAGS) & STARTED_BIT) == 0 {
                if (gate(sub, p, 1, 0) & 0xFF) == 0 {
                    return sub;
                }
                wr32(sub + SUB_FLAGS, rd32(sub + SUB_FLAGS) | SET_BIT);
            }
            let vt = rd32(this);
            let fetch: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(rd32(vt + FETCH_SLOT) as usize) };
            return fetch(this, p);
        }
        let r2 = poll(sub);
        if r2 == POLL_DONE {
            wr32(p + P_Z0, 0);
            wr32(p + P_Z1, 0);
            wrf(p + P_COPY, rdf(p + P_COPY));
            wr32(p + P_SLOT, 0);
            return sub;
        }
        if r2 != POLL_HOT {
            return sub;
        }
        let prog = rdf(this + PROG_OFF);
        if prog != rdf(lf_checker_rt::relocated(C_THREE)) {
            if prog >= rdf(lf_checker_rt::relocated(C_TWO)) {
                return dispatch(this, p, sub, K_BIG);
            }
            if !(prog >= rdf(lf_checker_rt::relocated(C_ONE))) {
                let direct = rd32(sub + DIRECT_OBJ);
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    unsafe { core::mem::transmute(rd32(direct + DIRECT_SLOT) as usize) };
                f(sub + DIRECT_OBJ, prog.to_bits(), 0);
                return sub;
            }
            return dispatch(this, p, sub, K_BIG);
        }
        dispatch(this, p, sub, K_TEN)
    }
});

/// Shared tail of the 0x384 dispatch: callee 4, the gate unless started,
/// then callee 5. Lives outside the export so both dispatch sites share it.
#[inline(always)]
unsafe fn dispatch(this: u32, p: u32, sub: u32, k: u32) -> u32 {
    unsafe {
        const SUB_FLAGS: u32 = 0x0C;
        const STARTED_BIT: u32 = 0x01;
        const SET_BIT: u32 = 0x02;
        const PARAM_OFF: u32 = 0x38;
        const POLL_DONE: u32 = 0x11A;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        if (lf_checker_rt::callee_thiscall!(4, u32, this, sub, p, rd32(this + PARAM_OFF), k)
            & 0xFF)
            != 0
        {
            return sub;
        }
        if (rd32(sub + SUB_FLAGS) & STARTED_BIT) == 0 {
            let vt = rd32(sub);
            let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(vt + 0x14) as usize);
            if (f(sub, p, 1, 0) & 0xFF) == 0 {
                return sub;
            }
            wr32(sub + SUB_FLAGS, rd32(sub + SUB_FLAGS) | SET_BIT);
        }
        lf_checker_rt::callee_thiscall!(5, u32, this, POLL_DONE, p)
    }
}
