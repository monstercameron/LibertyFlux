// original: 0x00E6DF70 construct_and_register_global_00e6df70

/// Construct the global object at 0x017ACC8C through a thiscall,
/// then register its cleanup callback 0x00E72EF0 through a cdecl call.
/// The constructor call uses ECX and no stack arguments; the registration call
/// uses one stack argument. Return the registration callee's EAX. The checker
/// scripts both callees and compares call order, registers and arguments.

lf_checker_rt::export!(cdecl, rw_construct_and_register_global_00e6df70() -> u32 {
    const OBJECT_VA: u32 = 0x017ACC8C;
        const CALLBACK_VA: u32 = 0x00E72EF0;
        let object = lf_checker_rt::relocated(OBJECT_VA);
        let _initialized = lf_checker_rt::callee_thiscall!(1, u32, object);
        let callback = lf_checker_rt::relocated(CALLBACK_VA);
        lf_checker_rt::callee_cdecl!(2, u32, callback)

});
