// original: 0x009B6FF0 NativeImpl_GET_CAM_STATE
/// Return the camera system's state word for the given selector.
///
/// Passes `sel` to the camera-system query and unwraps the returned object
/// to its state word. stdcall, one integer argument, no stores.
lf_checker_rt::export!(stdcall, rw_009B6FF0(sel: u32) -> u32 {
    unsafe {
        const CAM_SYS: u32 = 0x0128E400;
        let obj: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(CAM_SYS), sel);
        lf_checker_rt::callee_thiscall!(2, u32, obj)
    }
});
