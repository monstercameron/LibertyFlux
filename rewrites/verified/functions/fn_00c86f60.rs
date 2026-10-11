// original: 0x00c86f60 fn_00c86f60_forward_task_update

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_global_f32(file_va: u32) -> f32 {
    let bits = unsafe { read_u32(lf_checker_rt::relocated(file_va)) };
    f32::from_bits(bits)
}

/// Forward the single cdecl stack argument to a scripted thiscall helper with
/// the fixed task-table context in ECX, returning the helper's full EAX value.
const TASK_UPDATE_CONTEXT: u32 = 0x016FB6A0;
const TASK_UPDATE_CALLEE: u32 = 1;

fn forward_task_update(argument: u32, perturb_argument: bool) -> u32 {
    let forwarded = if perturb_argument { argument.wrapping_add(1) } else { argument };
    lf_checker_rt::callee_thiscall!(TASK_UPDATE_CALLEE, u32, lf_checker_rt::relocated(TASK_UPDATE_CONTEXT), forwarded)
}

lf_checker_rt::export!(cdecl, rw_00c86f60_forward_task_update(argument: u32) -> u32 {
    forward_task_update(argument, false)
});
