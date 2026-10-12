// original: 0x00B100C0 CDynamicEntity::~CDynamicEntity__deleting
/// Run the base dynamic-entity destructor, then conditionally invoke the
/// cleanup callee when the low bit of the flags argument is set. The object
/// arrives in ECX and flags is the one stack argument; the function returns
/// the object pointer and pops that argument.
lf_checker_rt::export!(thiscall, rw_00b100c0(object: u32, flags: u32) -> u32 {
    const DELETE_FLAG: u32 = 1;

    let _ = lf_checker_rt::callee_thiscall!(1, u32, object);
    if flags & DELETE_FLAG != 0 {
        let _ = lf_checker_rt::callee_cdecl!(2, u32, object);
    }
    object
});
