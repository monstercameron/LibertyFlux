// original: 0x00db1650 forward_to_global_ui_object
/// Pass an argument through to a method on a shared UI object.
///
/// The address of the shared object is fixed; the argument is forwarded
/// unchanged and the callee's return value becomes this function's result.
export!(cdecl, rw_00db1650(arg: u32) -> u32 {
    unsafe {
        const UI_OBJECT: u32 = 0x01981a4c;
        callee_thiscall!(1, u32, relocated(UI_OBJECT), arg)
    }
});
