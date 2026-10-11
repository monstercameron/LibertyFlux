// original: 0x00A34330 CEntity::call_manager_with_self

/// Calls the fixed manager helper with the entity as its one stack argument
/// and the manager object's address in ECX. The helper's return value remains
/// in EAX. This is a no-stack-argument thiscall wrapper; the helper consumes
/// the pushed entity pointer.
lf_checker_rt::export!(thiscall, rw_00a34330(this: u32) -> u32 {
    const MANAGER_CONTEXT: u32 = 0x0123_89e0;

    lf_checker_rt::callee_thiscall!(1, u32, MANAGER_CONTEXT, this)
});
