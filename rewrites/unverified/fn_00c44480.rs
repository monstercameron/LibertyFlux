// original: 0x00C44480 CIntermezzoEventPlayerJackedVehicle::vf1
/// Update the event, then count its children in state 0. The second helper
/// uses this+4 as its ECX object and one zero stack argument. Its signed-positive
/// test replaces AL while preserving the helper's upper 24 EAX bits.
lf_checker_rt::export!(thiscall, rw_00c44480(this: u32) -> u32 {
    const UPDATE_EVENT: u32 = 1;
    const CHILD_LIST: u32 = 4;
    const COUNT_CHILDREN: u32 = 2;
    lf_checker_rt::callee_thiscall!(UPDATE_EVENT, u32, this);
    let list = unsafe { ((this + CHILD_LIST) as *const u32).read_unaligned() };
    let raw = lf_checker_rt::callee_thiscall!(COUNT_CHILDREN, u32, list, 0);
    (raw & 0xffff_ff00) | u32::from((raw as i32) > 0)
});
