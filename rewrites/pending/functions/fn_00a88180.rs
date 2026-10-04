// original: 0x00a88180 dual_init_and_forward
/// Initialise two static objects, then forward `this` onward.
///
/// Calls the init helper (intercepted, thiscall/0) on the static objects
/// at the two fixed addresses, then tail-calls the shared successor on
/// `this` and returns its answer.
export!(thiscall, rw_00a88180(this_obj: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, relocated(0x128e310));
        let _: u32 = callee_thiscall!(2, u32, relocated(0x128e400));
        callee_thiscall!(3, u32, this_obj)
    }
});
