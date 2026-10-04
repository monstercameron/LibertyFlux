// original: 0x006986b0 guarded_shutdown_call
/// Run the shutdown helper on the second argument when non-null.
///
/// The first argument is ignored. Returns nothing observable: on the taken
/// path EAX holds the callee answer, otherwise the entry value.
export!(cdecl, rw_006986b0(_arg1: u32, arg2: u32) -> () {
    unsafe {
        if arg2 != 0 {
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(callee_addr(1) as usize);
            f(arg2, arg2);
        }
    }
});
