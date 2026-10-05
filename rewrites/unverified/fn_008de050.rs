// original: 0x008de050 CViewport_Callback_DC::vf1

/// Run the viewport callback with the summed offsets as its object.
///
/// Loads the callback from `this + 8` and tail-jumps to it with the object
/// pointer `this[0x18] + this[0x0c]` (callee 1); its answer is returned.
/// The rewrite performs the computed jump as a call that forwards the
/// object and the result, which is stack-neutral. Thiscall, no stack
/// arguments, one indirect outgoing call.
lf_checker_rt::export!(thiscall, rw_008de050(this: u32) -> u32 {
    unsafe {
        const CALLBACK_OFF: u32 = 8;
        const ADD_A_OFF: u32 = 0x0c;
        const ADD_B_OFF: u32 = 0x18;
        let target = ((this + CALLBACK_OFF) as *const u32).read_unaligned();
        let a = ((this + ADD_A_OFF) as *const u32).read_unaligned();
        let b = ((this + ADD_B_OFF) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        f(a.wrapping_add(b))
    }
});
