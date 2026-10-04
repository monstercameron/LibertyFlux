// original: 0x00DDE0F0 forward_to_inner_view
/// Forward one value to the inner view's update slot and return the value.
///
/// Loads the inner object at +0x1E8, calls through its function table at
/// +0x214 with the forwarded value, and returns that same value.
lf_checker_rt::export!(thiscall, rw_dde0f0(this: u32, value: u32) -> u32 {
    unsafe {
        let inner = *((this + 0x1E8) as *const u32);
        let vtable = *(inner as *const u32);
        let target = *((vtable + 0x214) as *const u32);
        let update: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let _ = update(inner, value);
        value
    }
});
