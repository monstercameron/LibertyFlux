// original: 0x00ca8e20 CEventHandler::vf53
/// Resolve the event's subject through three chained virtual calls, then
/// dispatch on the event code at `ev+0x10`.
///
/// `this` is the handler (thiscall: `this` in ecx, `ev` plus two unread
/// stack words; the callee pops 0xc bytes); `+0x04` is the owner, whose word at `+0x224`
/// is the inner object, and `+0x0c` takes the response.
///
/// Resolution: id 1 is the virtual slot at `[ev]+0x34` (fed the event);
/// a null answer returns zero. Otherwise id 2 is the slot at
/// `[[answer+0x224]]+0x1c` (fed `answer+0x224`); id 3 (direct, fed the id 2
/// answer plus `0x20`) is tested and a null answer returns zero. Then id 4
/// is the slot at `[id-3-answer]+0x34` (fed the id-3 answer); a null
/// answer returns zero. All three exits return zero because eax already
/// holds the tested null.
///
/// Dispatch decodes the code by repeated unsigned (wrapping) subtraction:
/// `0x1ab` allocates through id 8 and builds through id 9 (fed `0`,
/// `0x5f5e0ff` and `-1`), storing and returning the built object, or
/// storing and returning zero when allocation failed. `0x38d` and `0x38f`
/// share the response path: allocate through id 5, build through id 6
/// (fed the resolved subject and `0`), store at `this+0x0c`, set flag bit
/// 8 at `response+0x60` (faulting exactly like the original when
/// allocation or the build yielded null), then forward through id 7 (fed
/// the inner object, the subject and `1`) and return the forward answer.
/// Any other code returns the code minus `0x38f`, wrapping (the value left
/// by the third subtraction). The code tests are subtractions with zero
/// checks, so nothing here is signedness-sensitive.
lf_checker_rt::export!(thiscall, rw_00ca8e20(this: u32, ev: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const EVT_SLOT: u32 = 0x34;
        const LINK: u32 = 0x224;
        const LINK_SLOT: u32 = 0x1c;
        const ADJ: u32 = 0x20;
        const GATE_SLOT: u32 = 0x34;
        const CODE: u32 = 0x10;
        const OWNER: u32 = 0x04;
        const INNER: u32 = 0x224;
        const RESPONSE: u32 = 0x0c;
        const FLAG_OFF: u32 = 0x60;
        const FLAG_BIT: u32 = 8;
        const SHARED_GLOBAL: u32 = 0x0167_e2a0;
        const CODE_A: u32 = 0x1ab;
        const CODE_B_DELTA: u32 = 0x1e2;
        const CODE_C_DELTA: u32 = 2;
        const BUILD_MAGIC: u32 = 0x05f5_e0ff;
        const VFETCH: u32 = 1;
        const VLINK: u32 = 2;
        const ADJUST: u32 = 3;
        const VGATE: u32 = 4;
        const ALLOC: u32 = 5;
        const BUILD: u32 = 6;
        const FORWARD: u32 = 7;
        const ALLOC_A: u32 = 8;
        const BUILD_A: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let fetch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(ev) + EVT_SLOT) as usize);
        let subj = fetch(ev);
        if subj == 0 {
            return 0;
        }
        let link = rd32(subj + LINK);
        let get: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(link) + LINK_SLOT) as usize);
        let got: u32 = get(link);
        let adj: u32 = lf_checker_rt::callee_thiscall!(ADJUST, u32, got.wrapping_add(ADJ));
        if adj == 0 {
            return 0;
        }
        let gate: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(adj) + GATE_SLOT) as usize);
        if gate(adj) == 0 {
            return 0;
        }
        let global = rd32(lf_checker_rt::relocated(SHARED_GLOBAL));
        let c1 = rd32(ev + CODE).wrapping_sub(CODE_A);
        if c1 == 0 {
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC_A, u32, global);
            if obj == 0 {
                wr32(this + RESPONSE, 0);
                return 0;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(
                BUILD_A, u32, obj, 0u32, BUILD_MAGIC, 0xFFFF_FFFFu32
            );
            wr32(this + RESPONSE, r);
            return r;
        }
        let c2 = c1.wrapping_sub(CODE_B_DELTA);
        let c3 = c2.wrapping_sub(CODE_C_DELTA);
        if c2 != 0 && c3 != 0 {
            return c3;
        }
        let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, global);
        let r: u32 = if obj == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(BUILD, u32, obj, subj, 0u32)
        };
        wr32(this + RESPONSE, r);
        wr32(r + FLAG_OFF, rd32(r + FLAG_OFF) | FLAG_BIT);
        let owner = rd32(this + OWNER);
        let inner = rd32(owner + INNER);
        lf_checker_rt::callee_thiscall!(FORWARD, u32, inner, subj, 1u32)
    }
});
