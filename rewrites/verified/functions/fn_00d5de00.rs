// original: 0x00D5DE00 CPrtSysEntity::vf0

/// Release a particle-system entity and optionally notify its owning manager.
///
/// `this` is passed in ECX and one 32-bit release flag is passed on the stack.
/// The base entity cleanup helper always runs. If bit 0 of the release flag is
/// set, the function loads the manager pointer from the relocated manager
/// global and invokes its notification helper with the entity pointer. It
/// returns `this` and uses the 32-bit MSVC thiscall convention, cleaning the
/// one stack argument.
lf_checker_rt::export!(thiscall, rw_cprt_sys_entity_vf0(
    this_ptr: u32,
    release_flag: u32
) -> u32 {
    const BASE_CLEANUP: u32 = 1;
    const MANAGER_NOTIFICATION: u32 = 2;
    const MANAGER_GLOBAL_VA: u32 = 0x01723BD8;

    let _ = lf_checker_rt::callee_thiscall!(BASE_CLEANUP, u32, this_ptr);
    if release_flag & 1 != 0 {
        let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned() };
        let _ = lf_checker_rt::callee_thiscall!(MANAGER_NOTIFICATION, u32, manager, this_ptr);
    }
    this_ptr
});
