// original: 0x008C5710 proposed
/// Forward the wrapper's first stack argument in ECX and the next two as
/// thiscall stack arguments to the settings writer; preserve its return value.
lf_checker_rt::export!(cdecl, rw_008c5710(owner: u32, first: u32, second: u32) -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, owner, first, second)
});
