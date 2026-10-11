// original: 0x00A33F50 CEntity::vf15

/// Calls the fixed helper with the incoming word and a pointer to a saved
/// copy of `this`, then dispatches through the object's vtable slot at +0x38.
/// The helper writes through that frame-local pointer. The written word then
/// becomes the virtual call's single stack argument; `this` remains in ECX,
/// and the virtual call's EAX result is returned. This is a 32-bit thiscall
/// wrapper.
lf_checker_rt::export!(thiscall, rw_00a33f50(this: u32, value: u32) -> u32 {
    const VTABLE_SLOT: u32 = 0x38;

    let mut saved_this = this;
    let saved_this_ptr = (&mut saved_this as *mut u32) as u32;
    let _ = lf_checker_rt::callee_cdecl!(1, u32, value, saved_this_ptr);

    unsafe {
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable + VTABLE_SLOT) as *const u32;
        let target = slot.read_unaligned();
        let virtual_call: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        virtual_call(this, saved_this)
    }
});
