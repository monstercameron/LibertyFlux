// original: 0x00c44480 CIntermezzoEventPlayerJackedVehicle::vf1
/// Update the player-jacked event, then count state 0 on its child list.
/// Returns AL from the signed-positive count test and preserves the helper's
/// upper 24 EAX bits. The first thiscall receives the event as ECX; the
/// second receives the pointer at this+4 and one zero stack argument.
lf_checker_rt::export!(thiscall, rw_00c44480(this: u32) -> u32 {
    const UPDATE_EVENT: u32 = 1;
    const CHILD_LIST: u32 = 4;
    const COUNT_CHILDREN: u32 = 2;
    lf_checker_rt::callee_thiscall!(UPDATE_EVENT, u32, this);
    let list = unsafe { ((this + CHILD_LIST) as *const u32).read_unaligned() };
    let raw = lf_checker_rt::callee_thiscall!(COUNT_CHILDREN, u32, list, 0);
    (raw & 0xffff_ff00) | u32::from((raw as i32) > 0)
});
