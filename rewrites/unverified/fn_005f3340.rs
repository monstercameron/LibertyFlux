// original: 0x005f3340 fragInstNMGta::vf1

/// Adjusts this derived pointer back by `0x50`, reads the base object's vtable,
/// and tail-dispatches virtual slot `0xd0` with the adjusted pointer as `this`.
/// The selected method's return value is forwarded unchanged.
lf_checker_rt::export!(thiscall, rw_005f3340(this: u32) -> u32 {
    const BASE_ADJUST: u32 = 0x50;
    const VTABLE_SLOT: u32 = 0xd0;
    let base = this.wrapping_sub(BASE_ADJUST);
    let vtable = unsafe { (base as *const u32).read_unaligned() };
    let target = unsafe { (vtable.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned() };
    // SAFETY: the contract plants an intercepted thiscall stub in this vtable slot.
    let method: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(target as usize) };
    method(base)
});
