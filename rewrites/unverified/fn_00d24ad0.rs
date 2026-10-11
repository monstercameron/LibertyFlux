// original: 0x00D24AD0 CCustomShaderEffectVehicleFX::vf0
/// Runs the vehicle shader effect destructor and conditionally releases the object when the low
/// delete flag is set. The method receives one stack flag word, returns the object pointer, and
/// deletes only after destructor work completes.
lf_checker_rt::export!(thiscall, rw_00d24ad0(this: u32, delete_flags: u32) -> u32 {
    const DELETE_FLAG_MASK: u32 = 1;
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, (), this);
        if delete_flags & DELETE_FLAG_MASK != 0 {
            let _ = lf_checker_rt::callee_cdecl!(2, (), this);
        }
        this
    }
});
