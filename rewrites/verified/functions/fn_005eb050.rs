// original: 0x005EB050 CTaskComplexCop::vf6

/// Return the selected distance constant for the cop task's current target.
///
/// ECX points to the task and one unused stack word is removed on return. A
/// null target, or a non-null target whose virtual query returns `0x390`,
/// selects the default constant. Other query results select the alternate
/// constant. The virtual query is intercepted through the fabricated vtable
/// entry; the contract compares its receiver, answer and call record. Both
/// constants are read from the original image and returned as a single-precision
/// value in ST0. The query implementation is outside this function's proof.
lf_checker_rt::export!(thiscall, rw_005eb050(this: *mut u8, _unused: u32) -> f32 {
    const TARGET_POINTER_OFFSET: usize = 0x08;
    const TARGET_QUERY: u32 = 1;
    const QUERY_MATCH: u32 = 0x390;
    const DEFAULT_DISTANCE_VA: u32 = 0x00fe8b40;
    const ALTERNATE_DISTANCE_VA: u32 = 0x00fe8d94;

    unsafe {
        let target = this
            .add(TARGET_POINTER_OFFSET)
            .cast::<*mut u8>()
            .read_unaligned();
        let use_default = if target.is_null() {
            true
        } else {
            let target_address = target as u32;
            let query_result = lf_checker_rt::callee_thiscall!(TARGET_QUERY, u32, target_address);
            query_result == QUERY_MATCH
        };
        let constant_va = if use_default {
            DEFAULT_DISTANCE_VA
        } else {
            ALTERNATE_DISTANCE_VA
        };
        lf_checker_rt::global::<f32>(constant_va).read_unaligned()
    }
});
