// original: 0x00b3a2c0 task_gate_then_reposition (proposed)

/// Ask the gate callee about `obj`: when its low result byte is zero return
/// its full answer at once. Otherwise forward `obj`, the `key` word, the
/// `scale` word and the `mode` word to the reposition callee (key and scale
/// swap order across the call) and return its result. The reposition call
/// also carries the gate call's leftover register as one argument; that word
/// is unobservable residue, skipped in the contract. Original: 0x00b3a2c0
/// (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_00b3a2c0(obj: u32, key: u32, scale: u32, mode: u32) -> u32 {
    unsafe {
        const GATE: u32 = 1;
        const REPOSITION: u32 = 2;
        let gate: u32 = lf_checker_rt::callee_cdecl!(GATE, u32, obj);
        if gate & 0xff == 0 {
            return gate;
        }
        lf_checker_rt::callee_cdecl!(REPOSITION, u32, obj, scale, key, mode)
    }
});
