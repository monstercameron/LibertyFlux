// original: 0x009b74f0 NativeImpl_GET_ROOT_CAM
/// Feed two globals (argument word and object pointer) to the entity lookup,
/// store the answer into `*out`. Returns the answer.
/// (Engine getter behind GET_ROOT_CAM.)
export!(stdcall, rw_009b74f0(out: u32) -> u32 {
    unsafe {
        const ARG_G: u32 = 0x103E49C;
        const OBJ_G: u32 = 0x12FB1A0;
        let arg = *global::<u32>(ARG_G);
        let obj = *global::<u32>(OBJ_G);
        let ans = callee_thiscall!(1, u32, obj, arg);
        *(out as *mut u32) = ans;
        ans
    }
});
