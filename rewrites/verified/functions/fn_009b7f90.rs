// original: 0x009B7F90 dispatch_cam_instruction_target
/// Uses a direct helper to choose between a tail-dispatch path and an object-vtable path. The vtable path calls slot 2 with no stack argument, then slot 0 with the literal argument 1; the final call result is returned. The contract plants both slots as recorder stubs and exercises both branches.
lf_checker_rt::export!(thiscall, rw_009b7f90(this: u32, target: u32) -> u32 {
    const VTABLE: u32 = 0;
    const SLOT_ZERO: u32 = 0;
    const SLOT_TWO: u32 = 8;
    let selected = lf_checker_rt::callee_thiscall!(1, u32, this);
    if selected != 0 {
        return lf_checker_rt::callee_thiscall!(2, u32, selected);
    }
    unsafe {
        let vtable = (target as *const u32).read_unaligned();
        let first = (vtable.wrapping_add(SLOT_TWO) as *const u32).read_unaligned();
        let first_fn: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(first as usize);
        let _ = first_fn(target);
        let second = (vtable.wrapping_add(SLOT_ZERO) as *const u32).read_unaligned();
        let second_fn: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(second as usize);
        second_fn(target, 1)
    }
});
