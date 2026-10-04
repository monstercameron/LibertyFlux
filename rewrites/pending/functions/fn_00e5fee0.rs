// original: 0x00e5fee0 init_forward_ecx_and_register_teardown
/// Run a module initializer that takes the incoming register value,
/// then register its teardown callback.
///
/// The initializer receives this function's incoming ECX unchanged and
/// cleans it from the stack; returns the registration result.
export!(thiscall, rw_00e5fee0(passthrough: u32) -> u32 {
    unsafe {
        /// Teardown callback installed by this initializer.
        const TEARDOWN: u32 = 0x00E6F590;
        callee_stdcall!(1, u32, passthrough);
        callee_cdecl!(2, u32, relocated(TEARDOWN))
    }
});
