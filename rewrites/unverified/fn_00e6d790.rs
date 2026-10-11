// original: 0x00E6D790 initialize_and_register_00e6d790

/// Initialize one global resource and register its cleanup callback.
///
/// The first intercepted call receives the global object at 0x017A65B4 in ECX;
/// the second receives the relocated callback address 0x00E72E60 as its
/// only stack argument. The result is the second callee's EAX value. Both callees
/// are scripted by the checker, so this rewrite verifies the call sequence and
/// arguments while leaving their implementations outside this function proof.

lf_checker_rt::export!(cdecl, rw_initialize_and_register_00e6d790() -> u32 {
    unsafe {
        const OBJECT_VA: u32 = 0x017A65B4;
        const CALLBACK_VA: u32 = 0x00E72E60;
        let object = lf_checker_rt::relocated(OBJECT_VA);
        let _initialized = lf_checker_rt::callee_thiscall!(1, u32, object);
        let callback = lf_checker_rt::relocated(CALLBACK_VA);
        lf_checker_rt::callee_cdecl!(2, u32, callback)
    }
});
