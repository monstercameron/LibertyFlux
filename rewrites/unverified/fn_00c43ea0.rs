// original: 0x00C43EA0 CIntermezzoEventFallingThruAir::vf1
/// Count children in state 1 on the list pointer at offset 4. The helper uses
/// thiscall with one stack argument. The return is the signed-positive test in
/// AL; the helper's upper 24 EAX bits remain unchanged.
lf_checker_rt::export!(thiscall, rw_00c43ea0(this: u32) -> u32 {
    const CHILD_LIST: u32 = 4;
    const COUNT_CHILDREN: u32 = 1;
    let list = unsafe { ((this + CHILD_LIST) as *const u32).read_unaligned() };
    let raw = lf_checker_rt::callee_thiscall!(COUNT_CHILDREN, u32, list, 1);
    (raw & 0xffff_ff00) | u32::from((raw as i32) > 0)
});
