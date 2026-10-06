// original: 0x0099DDA0 audio_ensure_key_and_attach (proposed)

/// Ensure the entity has a key, then attach through the manager.
///
/// When `this`+0x9C is already nonzero the function returns at once with the entry `eax` (which the contract fixes to zero). Otherwise the candidate
/// from `this`+8 is examined: if its flag at +0x218 is set, or its flag at
/// +0x219 is clear, the probe is skipped; if neither, the probe (callee 1,
/// thiscall/0) runs and a zero answer stores the global fallback key and
/// returns it. From there the id helper (callee 2, thiscall/1 of constant
/// 1) and the manager attach (callee 3, thiscall/2 of the id and the
/// argument on the shared audio manager) run: a nonzero handle is attached
/// to the entity (callee 4, thiscall/1) and its answer returned, while a
/// zero handle is validated (callee 5, cdecl/1), falling back to the global
/// default key on failure. Thiscall with one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_0099DDA0(this: u32, arg: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x9C;
        const CANDIDATE: u32 = 8;
        const FLAG_A: u32 = 0x218;
        const FLAG_B: u32 = 0x219;
        const MANAGER: u32 = 0x01288780;
        const FALLBACK_KEY: u32 = 0x01284390;
        const DEFAULT_KEY: u32 = 0x012844B4;
        const PROBE_CALLEE: u32 = 1;
        const ID_CALLEE: u32 = 2;
        const ATTACH_CALLEE: u32 = 3;
        const SET_CALLEE: u32 = 4;
        const VALID_CALLEE: u32 = 5;
        let g = |va: u32| (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned();
        let mgr = lf_checker_rt::relocated(MANAGER);
        if ((this.wrapping_add(KEY) as *const u32).read_unaligned()) != 0 {
            return 0; // Entry eax, fixed to zero by the contract.
        }
        {
            let cand = ((this.wrapping_add(CANDIDATE)) as *const u32).read_unaligned();
            let fa = ((cand.wrapping_add(FLAG_A)) as *const u8).read();
            let mut probe = fa == 0;
            if probe {
                let fb = ((cand.wrapping_add(FLAG_B)) as *const u8).read();
                probe = fb != 0;
            }
            if probe {
                let ans = lf_checker_rt::callee_thiscall!(PROBE_CALLEE, u32, this);
                if (ans as u8) == 0 {
                    let fb = g(FALLBACK_KEY);
                    ((this.wrapping_add(KEY)) as *mut u32).write_unaligned(fb);
                    return fb;
                }
            }
        }
        let id = lf_checker_rt::callee_thiscall!(ID_CALLEE, u32, this, 1);
        let h = lf_checker_rt::callee_thiscall!(ATTACH_CALLEE, u32, mgr, id, arg);
        if h != 0 {
            return lf_checker_rt::callee_thiscall!(SET_CALLEE, u32, this, h);
        }
        let ok = lf_checker_rt::callee_cdecl!(VALID_CALLEE, u32,
            ((this.wrapping_add(KEY)) as *const u32).read_unaligned());
        if (ok as u8) != 0 {
            return ok;
        }
        let dflt = g(DEFAULT_KEY);
        ((this.wrapping_add(KEY)) as *mut u32).write_unaligned(dflt);
        dflt
    }
});
