// original: 0x00c76a50 resolve_model_class_resource

/// Look up the resource associated with the model-class descriptor and pass
/// the returned object to its refresh routine. Return the refresh routine's
/// EAX value. The descriptor is passed as a relocated image pointer to the
/// first stdcall call.
lf_checker_rt::export!(cdecl, rw_00c76a50() -> u32 {
    const MODEL_CLASS_DESCRIPTOR: u32 = 0x00c744c0;
    const LOOKUP_RESOURCE: u32 = 1;
    const REFRESH_RESOURCE: u32 = 2;
    let resource = lf_checker_rt::callee_stdcall!(
        LOOKUP_RESOURCE,
        u32,
        lf_checker_rt::relocated(MODEL_CLASS_DESCRIPTOR)
    );
    lf_checker_rt::callee_thiscall!(REFRESH_RESOURCE, u32, resource)
});
