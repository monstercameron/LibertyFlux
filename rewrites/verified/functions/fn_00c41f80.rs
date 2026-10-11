// original: 0x00c41f80 CIntermezzoEventGarage::vf5
/// Return AL=1 when there is no active root or any of the root's timing/state
/// gates block the event; otherwise query the final predicate and return AL=1
/// exactly when its EAX value is zero. In both cases preserve the upper EAX
/// bytes left by the last helper call, as the original does.
lf_checker_rt::export!(thiscall, rw_00c41f80(this: u32) -> u32 {
    const GET_ACTIVE_ROOT: u32 = 1;
    const FINAL_PREDICATE: u32 = 2;
    const ROOT_READY_BYTE: u32 = 0x210;
    const ROOT_STATE: u32 = 0xa74;
    const ROOT_LINK: u32 = 0x224;
    const ROOT_MODE: u32 = 0xa70;
    let root = lf_checker_rt::callee_cdecl!(GET_ACTIVE_ROOT, u32, );
    let upper = root & 0xffff_ff00;
    if root == 0 {
        return upper | 1;
    }
    unsafe {
        let root_bytes = root as *const u8;
        if root_bytes.add(ROOT_READY_BYTE as usize).read_unaligned() != 0 {
            return upper | 1;
        }
        let state = ((root + ROOT_STATE) as *const u32).read_unaligned();
        if state == 1 || state == 2 {
            return upper | 1;
        }
        if ((root + ROOT_LINK) as *const u32).read_unaligned() == 0 {
            return upper | 1;
        }
        if ((root + ROOT_MODE) as *const u32).read_unaligned() == 1 {
            return upper | 1;
        }
    }
    let predicate = lf_checker_rt::callee_cdecl!(FINAL_PREDICATE, u32, );
    (predicate & 0xffff_ff00) | u32::from(predicate == 0)
});
