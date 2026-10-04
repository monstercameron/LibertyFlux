// original: 0x00ca4fb0 CEventHandler::vf25
/// Event slot with two virtual kind probes (slot 1 on the event object).
/// The first probe gates on kind 0x1B with a null secondary pointer; a
/// global readiness check and a second probe gate on kind 0x30; then the
/// event type word selects: 0xC8 stores null, 0xE9 runs a two-stage lookup
/// plus factory conversion into this+0xC, anything else leaves this+0xC
/// alone. Returns the probe/lookup value seen on the path taken.
lf_rs75_rt::export!(thiscall, rw_00ca4fb0(this: u32, ev: u32, _b: u32, _c: u32) -> u32 {
    unsafe {
        let vtbl = *(ev as *const u32);
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vtbl + 4) as *const u32) as usize);
        let ebp = *((ev + 0x18) as *const u32);
        let ebx = *((ev + 0x1C) as *const u32);
        let k = probe(ev);
        if k == 0x1B && ebx == 0 {
            return k;
        }
        let ready: u32 = lf_rs75_rt::callee_thiscall!(2, u32, ev);
        if ready & 0xFF != 0 {
            let k2 = probe(ev);
            if k2 == 0x30 {
                return k2;
            }
        }
        let t = *((ev + 0x10) as *const u32);
        if t == 0xC8 {
            *((this + 0xC) as *mut u32) = 0;
            return 0;
        }
        if t.wrapping_sub(0xC8).wrapping_sub(0x21) != 0 {
            return t.wrapping_sub(0xE9);
        }
        let mid: u32 = lf_rs75_rt::callee_cdecl!(3, u32, ebp, ebx);
        let fin: u32 = lf_rs75_rt::callee_thiscall!(4, u32, mid, ebp, ebx);
        if fin & 0xFF == 0 {
            *((this + 0xC) as *mut u32) = 0;
            return fin;
        }
        let mgr = *lf_rs75_rt::global::<u32>(0x0167E2A0);
        let h: u32 = lf_rs75_rt::callee_thiscall!(5, u32, mgr);
        if h == 0 {
            *((this + 0xC) as *mut u32) = 0;
            return 0;
        }
        let ans: u32 = lf_rs75_rt::callee_thiscall!(6, u32, h, ebp, ebx);
        *((this + 0xC) as *mut u32) = ans;
        ans
    }
});
