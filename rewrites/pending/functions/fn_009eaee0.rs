// original: 0x009eaee0 CPlayerPed::vf19
/// Player-ped virtual slot 19: notify the two sub-objects in turn.
///
/// Calls slot 8 of the object at `0x7b0` with the argument when present,
/// then tail-calls slot 8 of the object at `0x7b4` when present (the
/// rewrite expresses the computed tail jump as a forwarding call with the
/// same object, argument and result). Returns the last answer produced,
/// or 0 when neither object was present (entry `eax`, fixed to 0 by the
/// contract, since it is not an argument).
export!(thiscall, rw_009eaee0(this_ptr: u32, arg: u32) -> u32 {
    unsafe {
        let mut answer = 0;
        let first = *((this_ptr + 0x7b0) as *const u32);
        if first != 0 {
            let vt = *(first as *const u32);
            let tgt = *((vt + 8) as *const u32);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            answer = f(first, arg);
        }
        let second = *((this_ptr + 0x7b4) as *const u32);
        if second != 0 {
            let vt = *(second as *const u32);
            let tgt = *((vt + 8) as *const u32);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            answer = f(second, arg);
        }
        answer
    }
});
