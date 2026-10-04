// original: 0x00a649b0 PedIntel_SetTarget
/// Retargets the tracked object, resolving it lazily through helpers.
///
/// When the context field at +0x40 already equals the new target, does
/// nothing. Otherwise, when the slot at +0x25C is empty, resolves it: the
/// locator helper runs on the shared global, and when it yields an object
/// the binder helper combines it with the old context. A resolved slot is
/// then notified through its handler slot 1 with the new target and the
/// extra argument. Returns nothing.
export!(thiscall, rw_00a649b0(this: u32, x: u32, y: u32) -> u32 {
    unsafe {
        if *((this + 0x40) as *const u32) == x {
            return 0;
        }
        if *((this + 0x25C) as *const u32) == 0 {
            let helper = callee_thiscall!(1, u32, *global::<u32>(0x171F9C0));
            let slot = if helper == 0 {
                0
            } else {
                callee_thiscall!(2, u32, helper, *((this + 0x40) as *const u32))
            };
            *((this + 0x25C) as *mut u32) = slot;
        }
        let obj = *((this + 0x25C) as *const u32);
        if obj == 0 {
            return 0;
        }
        type Notify = extern "thiscall" fn(u32, u32, u32) -> u32;
        let vtable = *(obj as *const u32);
        let notify: Notify =
            core::mem::transmute(*((vtable + 4) as *const u32) as usize);
        notify(obj, x, y);
    }
    0
});
