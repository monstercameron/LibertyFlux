// original: 0x00ca8280 CEventHandler::vf24
/// Handle the event whose subject pointer sits at `ev+0x0c`: build a response
/// through the allocator/builder pair, forward it to the owner's inner object,
/// then, when the handler has no follow-up state yet and the check passes,
/// build and store follow-up state.
///
/// `this` is the handler: word at `+0x04` is the owner, whose word at
/// `+0x224` is the inner object; `+0x0c` takes the response, `+0x08` the
/// follow-up state. `ev` is the event (thiscall: `this` in ecx, `ev` plus two
/// unread stack words; the callee pops 0xc bytes).
///
/// Paths: a null subject returns the caller's incoming eax, pinned by the
/// contract (the original never writes eax on that path). Otherwise the
/// response is allocated through id 1 and built through id 2 (fed the
/// subject and `0`); the built response is stored at `this+0x0c` and flag
/// bit 8 is set at `response+0x60`, faulting exactly like the original when
/// allocation or the build yielded null. The response is forwarded through
/// id 3 (fed the inner object, the subject and `1`). A nonzero `this+0x08`
/// then returns the forward answer; otherwise id 4 (cdecl, fed the owner
/// and the event) is tested on its low byte only and a zero low byte
/// returns the check's full answer. Then a second object is allocated
/// through id 6 (a null allocation stores zero at `this+0x08` and returns
/// zero), id 5 (stdcall, fed the owner, the subject and `0`) builds a
/// parameter, and id 7 (fed `1`, `1` and that parameter) builds the
/// follow-up state stored at `this+0x08`, which is also returned. All
/// comparisons are null/zero tests; nothing here is signedness-sensitive.
lf_checker_rt::export!(thiscall, rw_00ca8280(this: u32, ev: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const SUBJECT: u32 = 0x0c;
        const OWNER: u32 = 0x04;
        const INNER: u32 = 0x224;
        const RESPONSE: u32 = 0x0c;
        const STATE: u32 = 0x08;
        const FLAG_OFF: u32 = 0x60;
        const FLAG_BIT: u32 = 8;
        const SHARED_GLOBAL: u32 = 0x0167_e2a0;
        const EAX_PIN: u32 = 0x1357_9BDF;
        const ALLOC1: u32 = 1;
        const BUILD: u32 = 2;
        const FORWARD: u32 = 3;
        const CHECK: u32 = 4;
        const MAKE: u32 = 5;
        const ALLOC2: u32 = 6;
        const FINISH: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let subj = rd32(ev + SUBJECT);
        if subj == 0 {
            return EAX_PIN;
        }
        let global = rd32(lf_checker_rt::relocated(SHARED_GLOBAL));
        let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC1, u32, global);
        let r: u32 = if obj == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(BUILD, u32, obj, subj, 0u32)
        };
        wr32(this + RESPONSE, r);
        wr32(r + FLAG_OFF, rd32(r + FLAG_OFF) | FLAG_BIT);
        let owner = rd32(this + OWNER);
        let inner = rd32(owner + INNER);
        let fwd: u32 = lf_checker_rt::callee_thiscall!(FORWARD, u32, inner, subj, 1u32);
        if rd32(this + STATE) != 0 {
            return fwd;
        }
        let chk: u32 = lf_checker_rt::callee_cdecl!(CHECK, u32, owner, ev);
        if (chk & 0xFF) == 0 {
            return chk;
        }
        let obj2: u32 = lf_checker_rt::callee_thiscall!(ALLOC2, u32, global);
        if obj2 == 0 {
            wr32(this + STATE, 0);
            return 0;
        }
        let t: u32 = lf_checker_rt::callee_stdcall!(MAKE, u32, owner, subj, 0u32);
        let f: u32 = lf_checker_rt::callee_thiscall!(FINISH, u32, obj2, 1u32, 1u32, t);
        wr32(this + STATE, f);
        f
    }
});
