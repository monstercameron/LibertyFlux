// original: 0x00c43ea0 CIntermezzoEventFallingThruAir::vf1
/// Count one child state and return the signed-positive predicate in AL while
/// preserving the callee's upper 24 EAX bits. The helper receives the pointer
/// at this+4 in ECX and state value 1 on the stack.
lf_checker_rt::export!(thiscall, rw_00c43ea0(this: u32) -> u32 {
    const CHILD_LIST: u32 = 4;
    const COUNT_CHILDREN: u32 = 1;
    let list = unsafe { ((this + CHILD_LIST) as *const u32).read_unaligned() };
    let raw = lf_checker_rt::callee_thiscall!(COUNT_CHILDREN, u32, list, 1);
    (raw & 0xffff_ff00) | u32::from((raw as i32) > 0)
});
