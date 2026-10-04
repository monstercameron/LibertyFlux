// original: 0x00d28fb0 CPedTargetting::vf9 (symbols)

/// Forward to the installed targeting callback.
///
/// Calls the function pointer held at `this + 0x240` (planted by the
/// contract) with the handle at `this + 0x24c` and `arg`, and returns its
/// answer.
///
/// Original: 0x00D28FB0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00d28fb0(this: u32, arg: u32) -> u32 {
    unsafe {
        const CALLBACK_OFF: u32 = 0x240;
        const HANDLE_OFF: u32 = 0x24c;
        let target = unsafe { ((this + CALLBACK_OFF) as *const u32).read_unaligned() };
        let handle = unsafe { ((this + HANDLE_OFF) as *const u32).read_unaligned() };
        let f: extern "cdecl" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        f(handle, arg)
    }
});
