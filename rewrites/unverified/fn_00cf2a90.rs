// original: 0x00CF2A90 phMaterialMgrGta::destroy

/// Destroy the optional material manager and clear its global pointer.
///
/// `this` may be null. For a non-null object, its first word points to a
/// function table whose first entry is called as a thiscall destructor with
/// the single flag value 1. The global manager slot is then cleared. The
/// original is void; EAX is only constrained in this proof by a zero entry
/// value on the no-object path.
lf_checker_rt::export!(thiscall, rw_fn_00cf2a90(this: u32) -> u32 {
    const GLOBAL_MANAGER_VA: u32 = 0x018b_8968;
    let mut eax = 0;
    if this != 0 {
        let vtable = unsafe { (this as *const u32).read_unaligned() };
        let destructor = unsafe { (vtable as *const u32).read_unaligned() };
        let call: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(destructor as usize) };
        eax = call(this, 1);
    }
    unsafe { lf_checker_rt::global::<u32>(GLOBAL_MANAGER_VA).write_unaligned(0); }
    eax
});
