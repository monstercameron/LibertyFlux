// original: 0x00ca98e0 CEventHandler::vf47
/// Dispatch one event by its code: build a code-specific response object and
/// store it at `this+0x0c`, or forward unknown codes to the handler's parent
/// slot.
///
/// `this` is the handler: word at `+0x04` points to the owner, whose word at
/// `+0x224` is the inner object and whose word at `+0xb30` is the context
/// handed to most constructors. `ev` is the event, read only for its code
/// word at `+0x10`. The second and third stack arguments are not read.
/// Returns the stored response, a parent answer, the owner, the event, or
/// zero depending on the path. (thiscall:
/// `this` in ecx, three stack words.)
///
/// Codes `0x2c2`, `0x2c3`, `0x2d6` and `0x2e2` each allocate through id 1
/// (fed the shared global) and construct through their own callee
/// (ids 3, 4, 5, 6); the `0x2c2`/`0x2c3`/`0x2e2` cases return the owner when
/// the context is zero. Code `0xc8` stores zero and returns the event
/// pointer (whatever eax still holds). Codes
/// `0x76c` and `0x38f` build through id 8 (fed a lookup answer from the
/// one-argument cdecl id 2) and then share a tail that forwards through
/// id 9 (inner object, another lookup answer, `1`); the `0x38f` case also
/// sets flag `8` at `response+0x60`, faulting exactly like the original
/// when allocation failed. Code `0x38e` builds through id 10 with a float
/// bit constant (`0x447a0000`) and size `0x186a0` among its arguments.
/// Any other code calls the parent slot at `[this]+0x14c` (id 7, fed the
/// code) and returns its answer. A failed allocation stores zero and
/// returns zero, except on the `0x76c` path, which still runs the shared
/// tail, and the `0x38f` path, which faults on the flag write.
lf_checker_rt::export!(thiscall, rw_00ca98e0(this: u32, ev: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const OWNER: u32 = 0x04;
        const INNER: u32 = 0x224;
        const CONTEXT: u32 = 0xb30;
        const RESPONSE: u32 = 0x0c;
        const CODE: u32 = 0x10;
        const PARENT_SLOT: u32 = 0x14c;
        const LOOKUP_OFF: u32 = 0x20;
        const LOOKUP_ADVANCE: u32 = 0x30;
        const FLOAT_BITS: u32 = 0x447a_0000;
        const BUILD_SIZE: u32 = 0x186a0;
        const RESP_FLAG: u32 = 0x60;
        const RESP_BIT: u32 = 8;
        const SHARED_GLOBAL: u32 = 0x0167_e2a0;
        const ALLOC: u32 = 1;
        const LOOKUP: u32 = 2;
        const BUILD_2C2: u32 = 3;
        const BUILD_2C3: u32 = 4;
        const BUILD_2D6: u32 = 5;
        const BUILD_2E2: u32 = 6;
        const PARENT: u32 = 7;
        const BUILD_TAIL: u32 = 8;
        const FORWARD: u32 = 9;
        const BUILD_38E: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let code = rd32(ev + CODE);
        let owner = rd32(this + OWNER);
        let global = rd32(lf_checker_rt::relocated(SHARED_GLOBAL));

        if code == 0xc8 {
            wr32(this + RESPONSE, 0);
            return ev;
        }
        if code == 0x2c2 {
            if rd32(owner + CONTEXT) == 0 {
                return owner;
            }
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, global);
            if obj == 0 {
                wr32(this + RESPONSE, 0);
                return 0;
            }
            let t: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, 0u32);
            let p = rd32(t + LOOKUP_OFF).wrapping_add(LOOKUP_ADVANCE);
            let r: u32 = lf_checker_rt::callee_thiscall!(
                BUILD_2C2, u32, obj, rd32(owner + CONTEXT), p, 0, 0, 0
            );
            wr32(this + RESPONSE, r);
            return r;
        }
        if code == 0x2c3 {
            if rd32(owner + CONTEXT) == 0 {
                return owner;
            }
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, global);
            if obj == 0 {
                wr32(this + RESPONSE, 0);
                return 0;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(
                BUILD_2C3, u32, obj, rd32(owner + CONTEXT), 0, 0, 1
            );
            wr32(this + RESPONSE, r);
            return r;
        }
        if code == 0x2d6 {
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, global);
            if obj == 0 {
                wr32(this + RESPONSE, 0);
                return 0;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(BUILD_2D6, u32, obj, 0, 0);
            wr32(this + RESPONSE, r);
            return r;
        }
        if code == 0x2e2 {
            if rd32(owner + CONTEXT) == 0 {
                return owner;
            }
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, global);
            if obj == 0 {
                wr32(this + RESPONSE, 0);
                return 0;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(
                BUILD_2E2, u32, obj, rd32(owner + CONTEXT), 0, 0, 0
            );
            wr32(this + RESPONSE, r);
            return r;
        }
        if code == 0x76c || code == 0x38f {
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, global);
            if obj == 0 {
                if code == 0x38f {
                    wr32(this + RESPONSE, 0);
                    wr32(RESP_FLAG, rd32(RESP_FLAG) | RESP_BIT);
                    return 0;
                }
            } else {
                let t: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, 0u32);
                let r: u32 = lf_checker_rt::callee_thiscall!(BUILD_TAIL, u32, obj, t, 0);
                wr32(this + RESPONSE, r);
                if code == 0x38f {
                    wr32(r + RESP_FLAG, rd32(r + RESP_FLAG) | RESP_BIT);
                }
            }
            if obj == 0 && code == 0x76c {
                wr32(this + RESPONSE, 0);
            }
            let inner = rd32(owner + INNER);
            let t2: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, 0u32);
            let f: u32 = lf_checker_rt::callee_thiscall!(FORWARD, u32, inner, t2, 1);
            return f;
        }
        if code == 0x38e {
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, global);
            if obj == 0 {
                wr32(this + RESPONSE, 0);
                return 0;
            }
            let t: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, 0u32);
            let p = rd32(t + LOOKUP_OFF).wrapping_add(LOOKUP_ADVANCE);
            let r: u32 = lf_checker_rt::callee_thiscall!(
                BUILD_38E, u32, obj, p, 0, FLOAT_BITS, BUILD_SIZE, 0
            );
            wr32(this + RESPONSE, r);
            return r;
        }
        let parent: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(this) + PARENT_SLOT) as usize);
        parent(this, code)
    }
});
