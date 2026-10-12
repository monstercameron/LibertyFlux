// original: 0x00CF29C0 phMaterialMgrGta::~phMaterialMgrGta__deleting_3

/// Run the material-manager destructor and conditionally run its second
/// one-argument helper.
///
/// Thiscall receives `this` in ECX and a flags word on the stack. The
/// destructor is called first with `this`; when bit zero of `flags` is set, a
/// second cdecl helper receives `this`. The wrapper returns `this` in EAX and
/// removes its flags argument.
lf_checker_rt::export!(thiscall, rw_fn_00cf29c0(this: u32, flags: u32) -> u32 {
    const DESTRUCTOR_CALLEE: u32 = 1;
    const CONDITIONAL_CALLEE: u32 = 2;
    const DELETE_FLAG: u32 = 1;
    let _: u32 = lf_checker_rt::callee_thiscall!(DESTRUCTOR_CALLEE, u32, this);
    if flags & DELETE_FLAG != 0 {
        let _: u32 = lf_checker_rt::callee_cdecl!(CONDITIONAL_CALLEE, u32, this);
    }
    this
});
