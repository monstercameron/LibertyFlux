// original: 0x00953b60 bind_aux_view
/// Bind the object's auxiliary view through the two-stage binder.
///
/// The first stage receives the object and its parameter block; the second
/// receives the shared binder, the object's tag and the first stage's token.
/// Returns the second stage's answer.
export!(thiscall, rw_00953b60(obj: u32) -> u32 {
    unsafe {
        let params = obj.wrapping_add(0xC);
        let token: u32 = callee_thiscall!(0, u32, obj, params);
        let tag = *((obj.wrapping_add(8)) as *const u32);
        callee_thiscall!(1, u32, relocated(0x1284A60), tag, token)
    }
});
