// original: 0x00AF5EA0 resolve_task_property_or_default

/// Resolve a task property through a context object and return its stored
/// value. If the property lookup returns a null entry, return the relocated
/// address of the built-in default string instead. The helper call uses the
/// context in ECX and the property key as one stack argument.
lf_checker_rt::export!(cdecl, rw_00AF5EA0(context: u32, property_key: u32) -> u32 {
    const DEFAULT_VALUE_VA: u32 = 0x00EA_87CA;

    let entry = lf_checker_rt::callee_thiscall!(1, u32, context, property_key);
    if entry == 0 {
        lf_checker_rt::relocated(DEFAULT_VALUE_VA)
    } else {
        unsafe { (entry.wrapping_add(4) as *const u32).read_volatile() }
    }
});
