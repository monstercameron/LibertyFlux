// original: 0x00B27E70 stop_movement
/// Forward a blend request with two zeroed values and return the delegated
/// method's result. The receiver is passed in ECX; the delegate receives the
/// two zero words and the caller's request word on the stack.
lf_checker_rt::export!(thiscall, rw_b27e70(this: u32, request: u32) -> u32 {
    unsafe { lf_checker_rt::callee_thiscall!(1, u32, this, 0, 0, request) }
});
