// original: 0x00ca9780 CEventHandler::vf60
/// Vet one event for an event handler: check the event's subject, consult
/// two predicate callees, and either forward the event, scan the handler's
/// recent-event ring, or build a response object.
///
/// `this` is the handler: word at `+0x04` points to the owner, whose word at
/// `+0x224` is the inner object. `ev` is the event: word at `+0x10` is the
/// event code and word at `+0x1c` points to the subject, whose kind bits at
/// `+0x28` (mask `0x3c0`) must equal `0xc0`. The second and third stack
/// arguments are not read. (thiscall: `this` in ecx, three stack words.)
///
/// Returns the event pointer when there is no subject, the masked kind bits
/// when they mismatch, or a callee answer or scanned base pointer per path;
/// every early return hands back exactly what the original leaves in eax.
///
/// Stages: if the gate predicate (id 1, fed the subject) answers nonzero,
/// return its answer. If the route predicate (id 2, fed the event code)
/// answers nonzero, forward through id 3 (subject, `1`) and return its
/// answer. Otherwise, when the owner's flag byte at `+0xa60` is `1` and the
/// ready check (id 4) answers nonzero, scan the sixteen slots at
/// `inner+0x168`, counting slots that hold an object whose skip byte at
/// `+0x211` is clear, which passes the ready check again, and whose confirm
/// call (id 5, on `slot+0x224` advanced by `0x2e0`) answers nonzero; two or
/// more counted returns the scan base. Finally, when the event code is
/// `0x76c` and the kind bits still match, allocate a response (id 6, fed the
/// shared global), construct it through id 7 (subject, `0`) — a failed
/// allocation faults on the flag write below, exactly as the original —
/// store it at `this+0x0c`, forward through id 3 again, set flag `0x4000` at
/// `response+0x60`, and return the response.
lf_checker_rt::export!(thiscall, rw_00ca9780(this: u32, ev: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const OWNER: u32 = 0x04;
        const INNER: u32 = 0x224;
        const RESPONSE: u32 = 0x0c;
        const EVENT_CODE: u32 = 0x10;
        const SUBJECT: u32 = 0x1c;
        const KIND: u32 = 0x28;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_WANT: u32 = 0xc0;
        const WANT_CODE: u32 = 0x76c;
        const FLAG_BYTE: u32 = 0xa60;
        const SLOTS: u32 = 0x168;
        const SLOT_COUNT: u32 = 16;
        const SLOT_SKIP: u32 = 0x211;
        const SLOT_INNER: u32 = 0x224;
        const SLOT_ADVANCE: u32 = 0x2e0;
        const COUNT_LIMIT: u32 = 2;
        const RESP_FLAG: u32 = 0x60;
        const RESP_BIT: u32 = 0x4000;
        const SHARED_GLOBAL: u32 = 0x0167_e2a0;
        const GATE: u32 = 1;
        const ROUTE: u32 = 2;
        const FORWARD: u32 = 3;
        const READY: u32 = 4;
        const CONFIRM: u32 = 5;
        const ALLOC: u32 = 6;
        const BUILD: u32 = 7;

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

        let code = rd32(ev + EVENT_CODE);
        let subject = rd32(ev + SUBJECT);
        if subject == 0 {
            return ev;
        }
        if rd32(subject + KIND) & KIND_MASK != KIND_WANT {
            return rd32(subject + KIND) & KIND_MASK;
        }
        let owner = rd32(this + OWNER);
        let inner = rd32(owner + INNER);
        let g: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, inner, subject);
        if g & 0xff != 0 {
            return g;
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(ROUTE, u32, inner, code);
        if r != 0 {
            let f: u32 = lf_checker_rt::callee_thiscall!(FORWARD, u32, inner, subject, 1);
            return f;
        }
        let mut tail_eax: u32 = 0;
        if rd8(owner + FLAG_BYTE) == 1 {
            let d: u32 = lf_checker_rt::callee_thiscall!(READY, u32, owner);
            if d & 0xff != 0 {
                let base = inner.wrapping_add(SLOTS);
                let mut count: u32 = 0;
                let mut i: u32 = 0;
                while i < SLOT_COUNT {
                    let slot = rd32(base.wrapping_add(i.wrapping_mul(4)));
                    if slot != 0
                        && rd8(slot + SLOT_SKIP) == 0
                        && {
                            let q: u32 =
                                lf_checker_rt::callee_thiscall!(READY, u32, slot);
                            q & 0xff != 0
                        }
                    {
                        let c: u32 = lf_checker_rt::callee_thiscall!(
                            CONFIRM, u32,
                            rd32(slot + SLOT_INNER).wrapping_add(SLOT_ADVANCE)
                        );
                        if c != 0 {
                            count += 1;
                        }
                    }
                    i += 1;
                }
                if count >= COUNT_LIMIT {
                    return base;
                }
                tail_eax = base;
            } else {
                tail_eax = d;
            }
        }
        if code != WANT_CODE {
            return tail_eax;
        }
        if rd32(subject + KIND) & KIND_MASK != KIND_WANT {
            return rd32(subject + KIND) & KIND_MASK;
        }
        let global = rd32(lf_checker_rt::relocated(SHARED_GLOBAL));
        let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, global);
        let built: u32 = if obj == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(BUILD, u32, obj, subject, 0)
        };
        wr32(this + RESPONSE, built);
        lf_checker_rt::callee_thiscall!(FORWARD, u32, inner, subject, 1);
        let resp = rd32(this + RESPONSE);
        wr32(resp + RESP_FLAG, rd32(resp + RESP_FLAG) | RESP_BIT);
        resp
    }
});
